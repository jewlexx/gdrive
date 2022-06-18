mod auth;
mod net;

use std::net::SocketAddr;

use anyhow::Context;
use axum::{routing::get, Router};
use lazy_static::lazy_static;
use parking_lot::{const_mutex, Mutex};
use serde::Deserialize;
use tokio::sync::mpsc;

use net::get_loopback;

use auth::{callback, client::credentials::ClientInfo, redirect, user::UserCredentials};

type Sender = mpsc::UnboundedSender<RedirectQuery>;

lazy_static! {
    static ref CLIENT_INFO: ClientInfo = ClientInfo::new().expect("cannot create client info");
    static ref REDIRECT_ADDR: SocketAddr = get_loopback().expect("cannot get loopback address");
}

static CLOSE_SERVER: Mutex<Option<Sender>> = const_mutex(None);
static USER_CODE: Mutex<Option<String>> = const_mutex(None);

#[derive(Debug, Deserialize)]
pub struct RedirectQuery {
    pub code: Option<String>,
    pub error: Option<String>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::level_filters::STATIC_MAX_LEVEL)
        .init();
    // Initialize client info straight away
    lazy_static::initialize(&CLIENT_INFO);

    let app = Router::new()
        .route("/", get(redirect))
        .route("/callback", get(callback));

    let (tx, mut rx) = mpsc::unbounded_channel::<RedirectQuery>();

    *CLOSE_SERVER.lock() = Some(tx);

    let addr = REDIRECT_ADDR.to_owned();

    let address = addr.to_string();

    tracing::debug!("Listening on http://{address}");

    match open::that(&address) {
        Ok(()) => tracing::debug!("Opened login page in web browser"),
        Err(e) => tracing::error!("Failed to open login page in web browser: {}", e),
    }

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(async {
            let query = rx.recv().await;

            if let Some(query) = query {
                let code = query.code.expect("no code in query");
                tracing::info!("Got code: {}", code);
                *USER_CODE.lock() = Some(code);
            }
        })
        .await?;

    let user_code = &*USER_CODE.lock().clone().context("no user code")?;

    let redirect_uri = format!("http://{address}/");

    let user_credentials = UserCredentials::get_credentials(
        &CLIENT_INFO.credentials.client_id,
        &CLIENT_INFO.credentials.client_secret,
        user_code,
        redirect_uri,
    )
    .await?;

    tracing::info!("{:?}", user_credentials);

    Ok(())
}
