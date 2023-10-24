use bittorrent_starter_rust::bencode::decode_bencoded_value;
use bittorrent_starter_rust::torrent::Torrent;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about=None)]
enum Args {
    Decode { bencode: String },
    Info { torrent_file: String },
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

// Usage: your_bittorrent.sh decode "<encoded_value>"
fn main() {
    let arg = Args::parse();

    match &arg {
        Args::Decode { bencode } => {
            request_bencode_decode(bencode);
        }
        Args::Info { torrent_file } => {
            request_torrent_info(torrent_file);
        }
    }
}
