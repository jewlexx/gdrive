use axum::{
    extract::Query,
    http::Response,
    response::{IntoResponse, Redirect},
};

pub mod client;
mod error;
pub mod user;

pub use error::AuthError;

pub type AuthResult<T> = Result<T, AuthError>;

use crate::{RedirectQuery, CLOSE_SERVER};

use client::get_redirect;

pub async fn redirect() -> impl IntoResponse {
    let redirect_uri = get_redirect();

    Redirect::temporary(&redirect_uri)
}

pub async fn callback(query: Query<RedirectQuery>) -> impl IntoResponse {
    let query = query.0;
    if let Some(code) = query.code {
        let client_info = RedirectQuery {
            code: Some(code),
            error: None,
        };

        let tx = CLOSE_SERVER.lock().clone().expect("cannot close server");
        tx.send(client_info).expect("cannot close server");
        drop(tx);

        Response::new("Successfully redirected".into())
    } else {
        let body = include_str!("../../public/error.html").replace(
            "%error_msg%",
            &query.error.unwrap_or_else(|| "invalid code".into()),
        );

        Response::new(body)
    }
}
