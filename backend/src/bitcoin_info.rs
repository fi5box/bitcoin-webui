use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct BitcoinInfo {
    latest_blocks: u64,
    synchronized: String,
    connections: u64,
    connections_in: u64,
    connections_out: u64,
    difficulty: String,
    disk_usage: String,
    mempool: String,
    hash_rate: String,
}

impl Default for BitcoinInfo {
    fn default() -> Self {
        Self {
            latest_blocks: 0,
            synchronized: "0%".to_string(),
            connections: 0,
            connections_in: 0,
            connections_out: 0,
            difficulty: "0TH/s".to_string(),
            disk_usage: "0G".to_string(),
            mempool: "0M".to_string(),
            hash_rate: "0EH/s".to_string(),
        }
    }
}

pub fn update_bitcoin_info(state: Arc<RwLock<BitcoinInfo>>) {}
