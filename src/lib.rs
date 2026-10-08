pub use error::Error;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

mod error;

pub fn start(endpoints: &str) -> Result<(), Error> {
    let config: String = std::fs::read_to_string(endpoints).map_err(|_| Error::Read)?;
    let caddy: Caddy = toml::from_str(&config).map_err(|_| Error::Parse)?;

    match ureq::post("http://localhost:2019/load").send_json(&caddy) {
        Ok(_) => Ok(()),
        Err(ureq::Error::StatusCode(_)) => Err(Error::Server),
        Err(_) => Err(Error::Connection),
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Caddy {
    pub apps: Apps,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Apps {
    pub http: Http,
    pub tls: Tls,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Http {
    pub servers: HashMap<String, Server>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Server {
    pub listen: Vec<String>,
    pub routes: Vec<Route>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tls_connection_policies: Option<Vec<TlsConnectionPolicy>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Route {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#match: Option<Vec<Match>>,
    pub handle: Vec<Handler>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Match {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Handler {
    pub handler: String,
    #[serde(flatten)]
    pub config: HashMap<String, Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TlsConnectionPolicy {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#match: Option<MatchSni>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MatchSni {
    pub sni: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Tls {
    pub automation: Automation,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Automation {
    pub policies: Vec<Policy>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Policy {
    pub subjects: Vec<String>,
}
