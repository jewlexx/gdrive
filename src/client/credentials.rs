use serde::{Deserialize, Serialize};

const CLIENT_FILE: &[u8] = include_bytes!("../../client.json");

impl ClientInfo {
    pub fn new() -> Result<Self, serde_json::Error> {
        serde_json::from_slice(CLIENT_FILE)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClientInfo {
    #[serde(rename = "installed")]
    pub credentials: Credentials,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Credentials {
    pub client_id: String,
    pub project_id: String,
    pub auth_uri: String,
    pub token_uri: String,
    pub auth_provider_x509_cert_url: String,
    pub client_secret: String,
    pub redirect_uris: Vec<String>,
}
