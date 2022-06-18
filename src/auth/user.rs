use std::collections::HashMap;

use anyhow::Context;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use super::AuthResult;

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
        let client_info = json!({
            "grant_type": "authorization_code",
            "client_id": client_id,
            "client_secret": client_secret,
            "redirect_uri": url_escape::encode_component(&redirect_uri),
            "code": user_code,
        });

        println!("{}", &client_info.to_string());

        let response = reqwest::Client::new()
            .post("https://www.googleapis.com/oauth2/v4/token")
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
            tracing::error!("with code {response_code}: \n\n {desc}");
            panic!();
        }

        Ok(serde_json::from_value(response_json)?)
    }
}
