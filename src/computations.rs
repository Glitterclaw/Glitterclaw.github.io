use chrono::Utc;
use sha1::Sha1;
use sha2::{Digest, Sha256};

// This function generates two different types of hashes for the current time in UTC.
// The first hash is a SHA1 hash of the current time in seconds since the Unix epoch.
// The second hash is a SHA256 hash of the same data.
pub fn generate_assorted_hashes(input: &str) -> (String, String) {
    if input.is_empty() {
        let unix_time = Utc::now().timestamp_millis();
        (
            format!("{:x}", Sha1::digest(&unix_time.to_string().as_bytes())),
            format!("{:x}", Sha256::digest(&unix_time.to_string().as_bytes())),
        )
    } else {
        (
            format!("{:x}", Sha1::digest(input.as_bytes())),
            format!("{:x}", Sha256::digest(input.as_bytes())),
        )
    }
}

// This function converts a string to escaped utf-8 representation
pub fn string_to_utf8_escaped(input: &str) -> String {
    let mut result = "".to_string();
    for c in input.chars() {
        let c_u32 = c as u32;
        let encoded = if c_u32 <= 0xFFFF {
            format!("{:04X}", c_u32)
        } else if c_u32 <= 0xFFFFF {
            format!("{:05X}", c_u32)
        } else {
            format!("{:06X}", c_u32)
        };
        result.push_str("\\u");
        result.push_str(&encoded);
    }
    result
}

// This function converts escaped utf-8 string to a regular string
pub fn utf8_escaped_to_string(input: &str) -> String {
    let mut result = "".to_string();
    let mut i = 0;
    input.split("\\u").for_each(|chunk| {
        if i > 0 {
            let c = u32::from_str_radix(chunk, 16)
                .ok()
                .and_then(|hex_u32| char::from_u32(hex_u32))
                .unwrap_or_else(|| ' ');
            result.push(c);
        }
        i += 1;
    });
    result
}
