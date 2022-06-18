use crate::{CLIENT_INFO, CODE_CHALLENGE, REDIRECT_ADDR};

pub mod credentials;

pub fn get_redirect() -> String {
    let addr = REDIRECT_ADDR.to_string();
    let info = &CLIENT_INFO.credentials;
    let challenge = (*CODE_CHALLENGE.lock()).clone().unwrap().1;

    let query = format!(
        "client_id={client_id}&redirect_uri={uri}&response_type=code&scope={scopes}&code_challenge={code_challenge}&code_challenge_method=S256",
        scopes = url_escape::encode_component("https://www.googleapis.com/auth/drive"),
        uri = url_escape::encode_component(&format!("http://{addr}/callback")),
        client_id = info.client_id,
        code_challenge = challenge,
    );
    let mut url = CLIENT_INFO.credentials.auth_uri.clone();
    url.push('?');

    url_escape::encode_query_to_string(query, &mut url);

    println!("{}", url);

    url
}
