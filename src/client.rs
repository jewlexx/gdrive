use serde::{Deserialize, Serialize};

const CLIENT_FILE: &[u8] = include_bytes!("../client.json");

impl ClientInfo {
    pub fn new() -> Result<Self, serde_json::Error> {
        serde_json::from_slice(CLIENT_FILE)
    }
}

#[derive(Serialize, Deserialize)]
pub struct ClientInfo {
    #[serde(rename = "web")]
    web: Web,
}

#[derive(Serialize, Deserialize)]
pub struct Web {
    #[serde(rename = "client_id")]
    pub client_id: String,

    #[serde(rename = "project_id")]
    pub project_id: String,

    #[serde(rename = "auth_uri")]
    pub auth_uri: String,

    #[serde(rename = "token_uri")]
    pub token_uri: String,

    #[serde(rename = "auth_provider_x509_cert_url")]
    pub auth_provider_x509_cert_url: String,

    #[serde(rename = "client_secret")]
    pub client_secret: String,

    #[serde(rename = "redirect_uris")]
    pub redirect_uris: Vec<String>,

    #[serde(rename = "javascript_origins")]
    pub javascript_origins: Vec<String>,
}
