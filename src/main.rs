mod client;
mod net;

use std::net::SocketAddr;

use axum::{
    extract::Query,
    response::{IntoResponse, Redirect, Response},
    routing::get,
    Router,
};
use lazy_static::lazy_static;
use serde::Deserialize;
use tokio::sync::mpsc;

use client::{credentials::ClientInfo, AUTH_ENDPOINT};
use net::get_loopback;

type Sender = mpsc::UnboundedSender<RedirectQuery>;
type Receiver = mpsc::UnboundedReceiver<RedirectQuery>;

lazy_static! {
    static ref CLIENT_INFO: ClientInfo = ClientInfo::new().unwrap();
    static ref REDIRECT_ADDR: SocketAddr = get_loopback().unwrap();
}

static mut CLOSE_SERVER: Option<Sender> = None;

#[derive(Debug, Deserialize)]
struct RedirectQuery {
    pub code: Option<String>,
    pub error: Option<String>,
}

fn get_redirect(addr: &SocketAddr) -> String {
    let info = &CLIENT_INFO.credentials;
    let query = format!(
        "client_id={}&redirect_uri=http://{addr}/callback&response_type=code&access_type=offline&scope=https://www.googleapis.com/auth/drive",
        info.client_id
    );
    let mut url = String::from(AUTH_ENDPOINT);
    url.push('?');

    url_escape::encode_query_to_string(query, &mut url);

    url
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    // Initialize client info straight away
    lazy_static::initialize(&CLIENT_INFO);

    let app = Router::new()
        .route("/", get(redirect))
        .route("/callback", get(callback));

    let (tx, mut rx) = mpsc::unbounded_channel::<RedirectQuery>();

    unsafe {
        CLOSE_SERVER = Some(tx);
    }

    let addr = REDIRECT_ADDR.to_owned();

    println!("Listening on http://{}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(async {
            rx.recv().await;
        })
        .await?;

    Ok(())
}

async fn redirect() -> impl IntoResponse {
    let redirect_uri = get_redirect(&REDIRECT_ADDR);

    Redirect::temporary(&redirect_uri)
}

async fn callback(query: Query<RedirectQuery>) -> impl IntoResponse {
    let query = query.0;
    if let Some(code) = query.code {
        let client_info = RedirectQuery {
            code: Some(code),
            error: None,
        };

        unsafe {
            CLOSE_SERVER.clone().unwrap().send(client_info).unwrap();
        }

        Response::new("Successfully redirected".into())
    } else {
        let body = include_str!("../public/error.html").replace(
            "%error_msg%",
            &query.error.unwrap_or_else(|| "invalid code".into()),
        );

        Response::new(body)
    }
}
