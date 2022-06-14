use axum::{
    extract::Query,
    http::Response,
    response::{IntoResponse, Redirect},
};

use crate::{client::get_redirect, RedirectQuery, CLOSE_SERVER, REDIRECT_ADDR};

pub async fn redirect() -> impl IntoResponse {
    let redirect_uri = get_redirect(&REDIRECT_ADDR);

    Redirect::temporary(&redirect_uri)
}

pub async fn callback(query: Query<RedirectQuery>) -> impl IntoResponse {
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
