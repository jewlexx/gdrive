use std::collections::HashMap;

use anyhow::Context;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};

use crate::{CLIENT_INFO, CODE_CHALLENGE};

use super::AuthResult;

fn encode_base64_url(unencoded: String) -> String {
    let mut base64 = unencoded;

    // Converts base64 to base64url.
    base64 = base64.replace('+', "-");
    base64 = base64.replace('/', "_");
    // Strips padding.
    base64 = base64.replace('=', "");

    base64
}

pub fn get_challenge() -> AuthResult<(String, String)> {
    use rand::Rng;

    // Characters allowed to be used in the verifier
    const CHARACTERS: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-._~";
    let chars = CHARACTERS.chars().collect::<Vec<_>>();

    let mut rng = rand::thread_rng();
    let mut verifier = String::new();

    for _ in 0..128 {
        let index = rng.gen_range(0..chars.len());
        verifier.push(chars[index]);
    }

    let mut hasher = Sha256::new();
    hasher.update(verifier.as_bytes());

    let res = hasher.finalize();

    let digest = format!("{:X}", res);

    Ok((encode_base64_url(verifier), encode_base64_url(digest)))
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserCredentials {
    /// The token that your application sends to authorize a Google API request.
    pub access_token: String,
    /// The remaining lifetime of the access token in seconds.
    pub expires_in: i64,
    /// The type of token returned. At this time, this field's value is always set to Bearer.
    pub token_type: String,
    /// The scopes of access granted by the access_token expressed as a list of space-delimited, case-sensitive strings.
    pub scope: String,
    /// A token that you can use to obtain a new access token. Refresh tokens are valid until the user revokes access. Again, this field is only present in this response if you set the access_type parameter to offline in the initial request to Google's authorization server.
    pub refresh_token: String,
}

fn json_to_hashmap(lookup: Value) -> HashMap<String, Value> {
    let keys = lookup.as_object().unwrap().keys().collect::<Vec<_>>();

    let mut map = HashMap::new();

    for key in keys {
        let varname = key.to_owned();
        let value = &lookup[&varname];
        map.insert(varname, value.clone());
    }

    map
}

impl UserCredentials {
    pub async fn get_credentials(
        client_id: &str,
        client_secret: &str,
        user_code: &str,
        redirect_uri: String,
    ) -> AuthResult<Self> {
        let verifier = (*CODE_CHALLENGE.lock()).clone().unwrap().1;

        let client_info = json!({
            "grant_type": "authorization_code",
            "client_id": client_id,
            "client_secret": client_secret,
            "redirect_uri": url_escape::encode_component(&redirect_uri),
            "code_verifier": verifier,
            "code": user_code,
        });

        println!("{}", &client_info.to_string());

        let response = reqwest::Client::new()
            .post(&CLIENT_INFO.credentials.token_uri)
            .header(
                "Accept",
                "text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8",
            )
            .form(&json_to_hashmap(client_info))
            .send()
            .await?;

        let response_code = response.status();

        let response_json: Value = response.json().await?;

        if let Some(desc) = response_json
            .as_object()
            .context("cannot convert value to object")?
            .get("error_description")
        {
            tracing::error!("with code {response_code}:\n {desc}");
            panic!();
        }

        Ok(serde_json::from_value(response_json)?)
    }
}
