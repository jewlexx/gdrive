use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::REDIRECT_ADDR;

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

impl UserCredentials {
    pub async fn get_credentials(
        client_id: &str,
        client_secret: &str,
        user_code: &str,
    ) -> Result<Self, reqwest::Error> {
        let redirect_uri = REDIRECT_ADDR.to_string();
        let client_info = json!({
            "grant_type": "authorization_code",
            "client_id": client_id,
            "client_secret": client_secret,
            "redirect_uri": redirect_uri,
            "code": user_code,
        });

        let response = reqwest::Client::new()
            .post("https://oauth2.googleapis.com/token")
            .header("Content-Type", "application/x-www-form-urlencoded")
            .json(&client_info)
            .send()
            .await?
            .text()
            .await?;

        tracing::info!("{}", response);

        Ok(serde_json::from_str(&response).unwrap())
    }
}
