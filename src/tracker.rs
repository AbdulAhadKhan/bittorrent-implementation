use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use serde_bytes::ByteBuf;

#[derive(Debug, Serialize)]
pub struct TrackerRequest {
    pub peer_id: String,
    pub port: u16,
    pub uploaded: u64,
    pub downloaded: u64,
    pub left: usize,
    pub compact: u8,
}

#[derive(Debug, Deserialize)]
pub struct TrackerResponse {
    interval: usize,
    peers: ByteBuf,
}

impl TrackerRequest {
    pub async fn get(&self, url: &str, info_hash: &str) -> Result<TrackerResponse, anyhow::Error> {
        let params = serde_urlencoded::to_string(&self)?;
        let tracker_url = format!("{}?info_hash={}&{}", url, info_hash, params);

        let response = reqwest::get(&tracker_url).await?.bytes().await?;
        let response: TrackerResponse = serde_bencode::from_bytes(&response)?;

        anyhow::Ok(response)
    }
}
