use serde::{Deserialize, Serialize};

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
