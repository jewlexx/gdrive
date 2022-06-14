mod client;
mod net;

use std::{
    convert::Infallible,
    io::{self, Read, Write},
    net::{SocketAddr, TcpListener, TcpStream},
};

use axum::{
    extract::Query,
    response::{IntoResponse, Redirect, Response},
    routing::get,
    Router,
};
use lazy_static::lazy_static;
use tokio::sync::mpsc;

use client::{credentials::ClientInfo, AUTH_ENDPOINT};
use net::get_loopback;

type Sender = mpsc::UnboundedSender<RedirectQuery>;
type Receiver = mpsc::UnboundedReceiver<RedirectQuery>;

lazy_static! {
    static ref CLIENT_INFO: ClientInfo = ClientInfo::new().unwrap();
    static ref CLOSE_SERVER: (Sender, Receiver) = mpsc::unbounded_channel::<RedirectQuery>();
}

#[derive(Debug)]
struct RedirectQuery {
    pub code: Option<String>,
    pub error: Option<String>,
}

fn get_redirect() -> String {
    let info = &CLIENT_INFO.credentials;
    let query = format!(
        "client_id={}&redirect_uri=http://127.0.0.1&response_type=code&access_type=offline",
        info.client_id
    );
    let mut url = String::from(AUTH_ENDPOINT);
    url.push('?');

    url_escape::encode_query_to_string(query, &mut url);

    url
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize client info straight away
    lazy_static::initialize(&CLIENT_INFO);

    let app = Router::new()
        .route("/", get(redirect))
        .route("/callback", get(callback));

    // let listener = TcpListener::bind("127.0.0.1:0")?;

    let addr = get_loopback()?;

    tracing::debug!("Listening on http://{}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

async fn redirect() -> impl IntoResponse {
    let redirect_uri = get_redirect();

    Redirect::temporary(&redirect_uri)
}

async fn callback(Query(query): Query<RedirectQuery>) -> impl IntoResponse {
    if let Some(code) = query.code {
        let client_info = RedirectQuery {
            code: Some(code),
            error: None,
        };

        CLOSE_SERVER.0.send(client_info).unwrap();

        Response::new("Successfully redirected".into())
    } else {
        let body = include_str!("../public/error.html")
            .replace("%error_msg%", &query.error.unwrap_or("invalid code".into()));

        Response::new(body)
    }
}
