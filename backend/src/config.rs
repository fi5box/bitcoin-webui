use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    pub port: u16,
    pub request_timeout: u64,
    pub static_dir: String,
    // proxy bitcoin
    pub rpc_url: String,
    pub rpc_username: String,
    pub rpc_password: String,
    pub update_interval: u64,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            port: 8080,
            static_dir: "dist".to_string(),
            rpc_url: "http://bitcoin:8332/".to_string(),
            rpc_username: "bitcoin".to_string(),
            rpc_password: "bitcoin".to_string(),
            request_timeout: 10,
            update_interval: 10,
        }
    }
}

pub fn load_config(path: impl AsRef<Path>) -> Config {
    let s = fs::read_to_string(path)
        .map_err(|e| println!("read_to_string err: {e}"))
        .unwrap();
    let config: Config = toml::from_str(&s).unwrap();
    config
}
