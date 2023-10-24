use serde::{Deserialize, Serialize};
use serde_bytes::ByteBuf;
use sha1::{Digest, Sha1};

#[derive(Debug, Deserialize, Serialize)]
pub struct Torrent {
    pub announce: String,
    pub info: Info,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Info {
    pub name: String,
    pub length: usize,
    #[serde(rename = "piece length")]
    pub piece_length: usize,
    pub pieces: ByteBuf,
}

impl Torrent {
    pub fn new(path: &str) -> Result<Torrent, anyhow::Error> {
        let file = std::fs::read(path)?;
        let torrent_info: Torrent = serde_bencode::from_bytes(&file)?;
        return anyhow::Ok(torrent_info);
    }

    pub fn get_info_hash(&self) -> [u8; 20] {
        let mut hasher = Sha1::new();
        let bencode = serde_bencode::to_bytes(&self.info).unwrap();

        hasher.update(bencode);
        hasher.finalize().into()
    }

    pub fn get_pieces_hashes(&self) -> Vec<Vec<u8>> {
        let hashes_iterator = self.info.pieces.chunks(20);
        let hashes: Vec<Vec<u8>> = hashes_iterator.map(|hash| hash.into()).collect();

        return hashes;
    }
}
