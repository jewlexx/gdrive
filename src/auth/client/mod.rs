use crate::{CLIENT_INFO, REDIRECT_ADDR};

pub mod credentials;

pub const AUTH_ENDPOINT: &str = "https://accounts.google.com/o/oauth2/v2/auth";

pub fn get_redirect() -> String {
    let addr = REDIRECT_ADDR.to_string();
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
