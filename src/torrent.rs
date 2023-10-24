use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Torrent {
    pub announce: String,
    pub info: Info,
}

#[derive(Debug, Deserialize)]
pub struct Info {
    pub length: usize,
    pub name: String,
    #[serde(rename = "piece length")]
    pub piece_length: usize,
}

impl Torrent {
    pub fn info(path: &str) -> Result<Torrent, anyhow::Error> {
        let file = std::fs::read(path)?;
        let torrent_info: Torrent = serde_bencode::from_bytes(&file)?;
        return anyhow::Ok(torrent_info);
    }
}
