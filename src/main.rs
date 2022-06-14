mod auth;
mod client;
mod net;

use std::net::SocketAddr;

use axum::{routing::get, Router};
use lazy_static::lazy_static;
use parking_lot::{const_mutex, Mutex};
use serde::Deserialize;
use tokio::sync::mpsc;

use client::credentials::ClientInfo;
use net::get_loopback;

use crate::auth::{callback, redirect};

type Sender = mpsc::UnboundedSender<RedirectQuery>;

lazy_static! {
    static ref CLIENT_INFO: ClientInfo = ClientInfo::new().unwrap();
    static ref REDIRECT_ADDR: SocketAddr = get_loopback().unwrap();
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
    tracing_subscriber::fmt::init();
    // Initialize client info straight away
    lazy_static::initialize(&CLIENT_INFO);

    let app = Router::new()
        .route("/", get(redirect))
        .route("/callback", get(callback));

    let (tx, mut rx) = mpsc::unbounded_channel::<RedirectQuery>();

    *CLOSE_SERVER.lock() = Some(tx);

    let addr = REDIRECT_ADDR.to_owned();

    tracing::info!("Listening on http://{}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(async {
            let query = rx.recv().await;

            if let Some(query) = query {
                let code = query.code.unwrap();
                tracing::info!("Got code: {}", code);
                *USER_CODE.lock() = Some(code);
            }
        })
        .await?;

    Ok(())
}
