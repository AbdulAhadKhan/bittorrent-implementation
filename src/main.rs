use serde_json;
use serde_json::json;
use std::env;

#[allow(dead_code)]
fn decode_bencoded_value(encoded_value: &str) -> (serde_json::Value, &str) {
    match encoded_value.chars().next() {
        Some('0'..='9') => {
            // Example: "5:hello" -> "hello"
            if let Some((size, rest)) = encoded_value.split_once(":") {
                if let Ok(size) = size.parse::<usize>() {
                    let (word, rest) = rest.split_at(size);
                    (json!(word), rest)
                } else {
                    panic!("Error extracting word of size {}", size)
                }
            } else {
                panic!("Error parsing string")
            }
        }
        Some('i') => {
            // Example: "i-35e" -> "-35"
            if let Some((number, rest)) = encoded_value.split_at(1).1.split_once("e") {
                let number = number.parse::<i64>().unwrap();
                (json!(number), rest)
            } else {
                panic!("Error retrieving number")
            }
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
        let encoded_value = &args[2];
        let (decoded_value, _) = decode_bencoded_value(encoded_value);
        println!("{}", decoded_value.to_string());
    } else {
        println!("unknown command: {}", args[1])
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use serde_json::json;

    fn decode_bencode_test_wrapper(input: &str, expected: (serde_json::Value, &str)) {
        let actual = decode_bencoded_value(input);
        assert_eq!(expected, actual);
    }

    #[test]
    fn decode_bencode_string() {
        decode_bencode_test_wrapper("5:hello", (json!("hello"), ""));
        decode_bencode_test_wrapper("12:hello world!", (json!("hello world!"), ""));
        decode_bencode_test_wrapper("7:consumeremain", (json!("consume"), "remain"))
    }

    #[test]
    fn decode_bencode_integer() {
        decode_bencode_test_wrapper("i36e", (json!(36), ""));
        decode_bencode_test_wrapper("i-36e", (json!(-36), ""));
        decode_bencode_test_wrapper("i10eremain", (json!(10), "remain"));
    }
}
