use sha1::{Digest, Sha1};
use std::time::{SystemTime, UNIX_EPOCH};

// Not really a uuid but random enough for our case. Also embarrassingly hacky 🤦.
// To keep it compatible with the request format,
// this will be 20 character long string.
pub fn generate_uuid() -> String {
    let mut hash = String::from("");

    for _ in 0..=5 {
        let time = SystemTime::now();
        let unix_nano_time = time.duration_since(UNIX_EPOCH).unwrap().as_nanos();

        let mut hasher = Sha1::new();
        let hash_me = format!("{}{}", hash, unix_nano_time);

        hasher.update(hash_me);
        let result = hex::encode(hasher.finalize());

        hash = result.clone();
    }

    hash
}

pub fn byte_url_encode(byte_array: &[u8]) -> String {
    let mut encoded_bytes = String::with_capacity(byte_array.len() * 3);

    for &byte in byte_array {
        let encoded_byte = hex::encode(&[byte]);
        encoded_bytes.push('%');
        encoded_bytes.push_str(&encoded_byte);
    }

    encoded_bytes
}
