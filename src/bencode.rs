use anyhow::anyhow;
use serde_json;

pub fn decode_bencoded_value(
    encoded_value: &str,
) -> Result<(serde_json::Value, &str), anyhow::Error> {
    match encoded_value.chars().next() {
        Some('0'..='9') => {
            // Example: "5:hello" -> "hello"
            match encoded_value.split_once(":") {
                Some((size, rest)) => {
                    let size = size.parse::<usize>()?;
                    if size > rest.len() {
                        return Err(anyhow!(
                            "Specified string '{}' is greater than size of {}",
                            rest,
                            size
                        ));
                    }
                    let (word, rest) = rest.split_at(size);
                    anyhow::Ok((word.into(), rest))
                }
                None => Err(anyhow!("Invalid bencode syntax: {}", encoded_value)),
            }
        }
        Some('i') => {
            // Example: "i-35e" -> "-35"
            match encoded_value.split_at(1).1.split_once("e") {
                Some((number, rest)) => {
                    let number = number.parse::<i64>()?;
                    anyhow::Ok((number.into(), rest))
                }
                None => Err(anyhow!("Invalid bencode syntax: {}", encoded_value)),
            }
        }
        Some('l') => {
            // Example: l5:helloi20ee -> ["hello", 20]
            let mut jsonable_list = Vec::new();
            let mut rest = encoded_value.split_at(1).1;
            while !rest.is_empty() && !rest.starts_with("e") {
                let (value, remaining) = decode_bencoded_value(rest)?;

                rest = remaining;
                jsonable_list.push(value);
            }
            anyhow::Ok((jsonable_list.into(), &rest[1..]))
        }
        Some('d') => {
            // Example: d5:hello5world7:versioni15ee -> {"hello": "world", "version": 15}
            let mut jsonable_dict = serde_json::Map::new();
            let mut rest = encoded_value.split_at(1).1;
            while !rest.is_empty() && !rest.starts_with("e") {
                let (key, remaining) = decode_bencoded_value(rest)?;
                let (value, remaining) = decode_bencoded_value(remaining)?;

                let key = match key {
                    serde_json::value::Value::String(key) => key,
                    _ => key.to_string(),
                };

                rest = remaining;
                jsonable_dict.insert(key, value);
            }
            anyhow::Ok((jsonable_dict.into(), &rest[1..]))
        }
        _ => Err(anyhow!("Unhandled encoded value: {}", encoded_value)),
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

    #[test]
    fn decode_bencoded_list() {
        let test_1_bencode = "l5:hello5:worlde";
        let test_1_results = json!(["hello", "world"]);
        assert_eq!(
            decode_bencoded_value(test_1_bencode).unwrap(),
            (test_1_results, "")
        );

        let test_2_bencode = "li20ei-15ee";
        let test_2_results = json!([20, -15]);
        assert_eq!(
            decode_bencoded_value(test_2_bencode).unwrap(),
            (test_2_results, "")
        );

        let test_3_bencode = "l5:helloi-15ee";
        let test_3_results = json!(["hello", -15]);
        assert_eq!(
            decode_bencoded_value(test_3_bencode).unwrap(),
            (test_3_results, "")
        );
    }

    #[test]
    fn decode_bencoded_dict() {
        let bencode = "d5:helloi52ee";
        let results = json!({"hello": 52});
        decode_bencode_test_wrapper(bencode, (results, ""));

        let bencode = "d3:foo3:bar5:helloi52ei10ei-5ee";
        let results = json!({"foo": "bar", "hello": 52, "10": -5});
        decode_bencode_test_wrapper(bencode, (results, ""));

        let bencode = "d5:helloi52eei1e";
        let results = json!({"hello": 52});
        decode_bencode_test_wrapper(bencode, (results, "i1e"));

        let bencode = "de";
        let results = json!({});
        decode_bencode_test_wrapper(bencode, (results, ""));
    }
}
