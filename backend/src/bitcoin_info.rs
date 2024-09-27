use bitcoincore_rpc::{Client, RpcApi};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct BitcoinInfo {
    latest_blocks: u64,
    difficulty: String,
    synchronized: String,
    disk_usage: String,
    prune_mode: String,
    connections: u64,
    connections_in: u64,
    connections_out: u64,
    mempool: String,
    hash_rate: String,
}

impl Default for BitcoinInfo {
    fn default() -> Self {
        Self {
            latest_blocks: 0,
            difficulty: "0 TH/s".to_string(),
            synchronized: "0%".to_string(),
            disk_usage: "0 GB".to_string(),
            prune_mode: "No".to_string(),
            connections: 0,
            connections_in: 0,
            connections_out: 0,
            mempool: "0 MB".to_string(),
            hash_rate: "0 EH/s".to_string(),
        }
    }
}

fn format_bytes(bytes: u64) -> String {
    let units = ["B", "KB", "MB", "GB", "TB", "PB"];
    if bytes == 0 {
        return "0B".to_string();
    }
    let mut unit_index = 0;
    let mut value = bytes as f64;
    while value >= 1024.0 && unit_index < units.len() - 1 {
        value /= 1024.0;
        unit_index += 1;
    }
    format!("{:.2} {}", value, units[unit_index])
}

fn format_hashrate(hs: f64) -> String {
    let units = ["H/s", "KH/s", "MH/s", "GH/s", "TH/s", "PH/s", "EH/s"];

    let mut unit_index = 0;
    let mut value = hs;
    while value >= 1000.0 && unit_index < units.len() - 1 {
        value /= 1000.0;
        unit_index += 1;
    }
    format!("{:.2} {}", value, units[unit_index])
}

pub async fn update_bitcoin_info(client: &Client, state: Arc<RwLock<BitcoinInfo>>) {
    let mut bitcoin_info = state.read().await.clone();
    if let Ok(blockchain_info) = client.get_blockchain_info() {
        bitcoin_info.latest_blocks = blockchain_info.headers;
        bitcoin_info.difficulty = format_hashrate(blockchain_info.difficulty);
        bitcoin_info.synchronized =
            format!("{:.2}%", blockchain_info.verification_progress * 100.0);
        bitcoin_info.disk_usage = format_bytes(blockchain_info.size_on_disk);
        if blockchain_info.pruned {
            bitcoin_info.prune_mode = "Yes".to_string();
        }
    }
    if let Ok(network_info) = client.get_network_info() {
        bitcoin_info.connections = network_info.connections as u64;
        if let Some(connections_in) = network_info.connections_in {
            bitcoin_info.connections_in = connections_in as u64;
        }
        if let Some(connections_out) = network_info.connections_out {
            bitcoin_info.connections_out = connections_out as u64;
        }
    }
    if let Ok(hps) = client.get_network_hash_ps(None, None) {
        bitcoin_info.hash_rate = format_hashrate(hps);
    }
    if let Ok(mempool_info) = client.get_mempool_info() {
        bitcoin_info.mempool = format_bytes(mempool_info.usage as u64);
    }

    *state.write().await = bitcoin_info;
}
