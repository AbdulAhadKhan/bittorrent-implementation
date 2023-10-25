use serde::{Deserialize, Serialize};
use serde_bytes::ByteBuf;
use std::convert::TryInto;
use std::net::Ipv4Addr;

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
    pub interval: usize,
    pub peers: ByteBuf,
}

type AddressList = Vec<(Ipv4Addr, u16)>;

impl TrackerRequest {
    pub async fn get(&self, url: &str, info_hash: &str) -> Result<TrackerResponse, anyhow::Error> {
        let params = serde_urlencoded::to_string(&self)?;
        let tracker_url = format!("{}?info_hash={}&{}", url, info_hash, params);

        let response = reqwest::get(&tracker_url).await?.bytes().await?;
        let response: TrackerResponse = serde_bencode::from_bytes(&response)?;

        anyhow::Ok(response)
    }
}

impl TrackerResponse {
    pub fn get_peers_address(&self) -> Result<AddressList, anyhow::Error> {
        let mut addresses: AddressList = Vec::new();

        for i in 0..(&self.peers.len() / 6) {
            let increment = i * 6;
            let end = increment + 6;

            let [a, b, c, d, port_1, port_2]: [_; 6] = self.peers[increment..end].try_into()?;
            let ipv4 = Ipv4Addr::new(a, b, c, d);
            let port = u16::from_be_bytes([port_1, port_2]);

            addresses.push((ipv4, port));
        }

        anyhow::Ok(addresses)
    }
}
