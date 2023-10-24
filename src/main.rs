use bittorrent_starter_rust::bencode::decode_bencoded_value;
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

// Usage: your_bittorrent.sh decode "<encoded_value>"
fn main() {
    let arg = Args::parse();

    match &arg {
        Args::Decode { bencode } => {
            request_bencode_decode(bencode);
        }
        Args::Info { torrent_file } => {
            println!("{:?}", torrent_file);
        }
    }
}
