use anyhow::anyhow;
use serde_json;
use serde_json::json;
use std::env;

#[allow(dead_code)]
fn decode_bencoded_value(encoded_value: &str) -> Result<(serde_json::Value, &str), anyhow::Error> {
    match encoded_value.chars().next() {
        Some('0'..='9') => {
            // Example: "5:hello" -> "hello"
            match encoded_value.split_once(":") {
                Some((size, rest)) => {
                    let size = size.parse::<usize>()?;
                    let (word, rest) = rest.split_at(size);
                    anyhow::Ok((json!(word), rest))
                }
                None => Err(anyhow!("Invalid bencode syntax: {}", encoded_value)),
            }
        }
        Some('i') => {
            // Example: "i-35e" -> "-35"
            match encoded_value.split_at(1).1.split_once("e") {
                Some((number, rest)) => {
                    let number = number.parse::<i64>()?;
                    anyhow::Ok((json!(number), rest))
                }
                None => Err(anyhow!("Invalid bencode syntax: {}", encoded_value)),
            }
        }
        _ => Err(anyhow!("Unhandled encoded value: {}", encoded_value)),
    }
}

// Usage: your_bittorrent.sh decode "<encoded_value>"
fn main() {
    let args: Vec<String> = env::args().collect();
    let command = &args[1];

    if command == "decode" {
        let encoded_value = &args[2];
        let decoded_value = decode_bencoded_value(encoded_value).unwrap();
        println!("{}", decoded_value.1.to_string());
    } else {
        println!("unknown command: {}", args[1])
    }
}

#[cfg(test)]
mod bencode_tests {
    use super::*;
    use serde_json::json;

    fn decode_bencode_test_wrapper(input: &str, expected: (serde_json::Value, &str)) {
        let actual = decode_bencoded_value(input);
        assert_eq!(expected, actual.unwrap());
    }

    #[test]
    fn decode_bencode_string() {
        decode_bencode_test_wrapper("5:hello", (json!("hello"), ""));
        decode_bencode_test_wrapper("12:hello world!", (json!("hello world!"), ""));
        decode_bencode_test_wrapper("7:consumeremain", (json!("consume"), "remain"));
    }

    #[test]
    fn decode_bencode_string_error() {
        assert!(decode_bencoded_value("5:").is_err());
        assert!(decode_bencoded_value(":fail").is_err());
    }

    #[test]
    fn decode_bencode_integer() {
        decode_bencode_test_wrapper("i36e", (json!(36), ""));
        decode_bencode_test_wrapper("i-36e", (json!(-36), ""));
        decode_bencode_test_wrapper("i10eremain", (json!(10), "remain"));
    }

    #[test]
    fn decode_bencode_integer_error() {
        assert!(decode_bencoded_value("ie").is_err());
        assert!(decode_bencoded_value("i-e").is_err());
    }
}
