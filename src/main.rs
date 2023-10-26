use bittorrent_starter_rust::bencode::decode_bencoded_value;
use bittorrent_starter_rust::handshake::Handshake;
use bittorrent_starter_rust::torrent::Torrent;
use bittorrent_starter_rust::tracker::{self, TrackerRequest};
use bittorrent_starter_rust::utils::{self, byte_url_encode, generate_uuid};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about=None)]
enum Args {
    Decode {
        bencode: String,
    },
    Info {
        torrent_file: String,
    },
    Peers {
        torrent_file: String,
    },
    Handshake {
        torrent_file: String,
        address: String,
    },
}

fn request_bencode_decode(encoded_value: &str) {
    let decoded_value = decode_bencoded_value(encoded_value).unwrap();
    println!("{}", decoded_value.0.to_string());
}

fn request_torrent_info(path: &str) {
    let torrent_info = Torrent::new(path).unwrap();
    let info_hash = torrent_info.get_info_hash();
    let info_hash = hex::encode(info_hash);
    let pieces_hases = torrent_info.get_pieces_hashes();

    println!("Tracker URL: {}", torrent_info.announce);
    println!("Length: {}", torrent_info.info.length);
    println!("Info Hash: {}", info_hash);
    println!("Piece Length: {}", torrent_info.info.piece_length);
    println!("Pieces Hashes:");
    for i in 0..pieces_hases.len() {
        let hash_hex = hex::encode(&pieces_hases[i]);
        println!("{}", hash_hex);
    }
}

async fn request_peers_info(torrent_file: &str) -> Result<tracker::AddressList, anyhow::Error> {
    let peer_id = generate_uuid();
    let torrent_info = Torrent::new(torrent_file)?;

    let announce_url = &torrent_info.announce;
    let info_hash = torrent_info.get_info_hash();

    let tracker_request = TrackerRequest {
        compact: 1,
        downloaded: 0,
        left: torrent_info.info.length,
        peer_id,
        port: 6881,
        uploaded: 0,
    };

    let tracker_response = tracker_request
        .get(announce_url, byte_url_encode(&info_hash).as_str())
        .await?;

    tracker_response.get_peers_address()
}

#[allow(unused)]
async fn request_peer_handshake(torrent_file: &str, address: &str) {}

#[tokio::main]
async fn main() {
    let arg = Args::parse();

    match &arg {
        Args::Decode { bencode } => {
            request_bencode_decode(bencode);
        }
        Args::Info { torrent_file } => {
            request_torrent_info(torrent_file);
        }
        Args::Peers { torrent_file } => {
            let address_list = request_peers_info(torrent_file).await.unwrap();

            for (ip, port) in address_list {
                println!("{}:{}", ip, port);
            }
        }
        Args::Handshake {
            torrent_file,
            address,
        } => {
            request_peer_handshake(torrent_file, address).await;
        }
    }
}
