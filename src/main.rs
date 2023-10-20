use serde_json;
use std::env;

#[allow(dead_code)]
fn decode_bencoded_value(encoded_value: &str) -> serde_json::Value {
    match encoded_value.chars().next() {
        Some('0'..='9') => {
            // Example: "5:hello" -> "hello"
            let colon_index = encoded_value.find(':').unwrap();
            let number_string = &encoded_value[..colon_index];
            let number = number_string.parse::<i64>().unwrap();
            let string = &encoded_value[colon_index + 1..colon_index + 1 + number as usize];
            return serde_json::Value::String(string.to_string());
        }
        Some('i') => {
            // Example: "i-35e" -> "-35"
            let end_index = encoded_value.find('e').unwrap();
            let encoded_number = &encoded_value[1..end_index];
            let number: i64 = encoded_number.parse().unwrap();
            let serde_number = serde_json::Number::from(number);
            return serde_json::Value::Number(serde_number);
        }
        _ => {
            panic!("Unhandled encoded value: {}", encoded_value)
        }
    }
}

// Usage: your_bittorrent.sh decode "<encoded_value>"
fn main() {
    let args: Vec<String> = env::args().collect();
    let command = &args[1];

    if command == "decode" {
        // You can use print statements as follows for debugging, they'll be visible when running tests.
        // println!("Logs from your program will appear here!");

        // Uncomment this block to pass the first stage
        let encoded_value = &args[2];
        let decoded_value = decode_bencoded_value(encoded_value);
        println!("{}", decoded_value.to_string());
    } else {
        println!("unknown command: {}", args[1])
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn decode_bencode_string() {
        assert_eq!(decode_bencoded_value("5:hello"), "hello")
    }

    #[test]
    fn decode_bencode_integer() {
        assert_eq!(decode_bencoded_value("i32e"), 32);
        assert_eq!(decode_bencoded_value("i-32e"), -32);
    }
}
