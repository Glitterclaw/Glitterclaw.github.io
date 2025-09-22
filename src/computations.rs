use chrono::Utc;
use sha1::Sha1;
use sha2::{Digest, Sha256};

// This function generates two different types of hashes for the current time in UTC.
// The first hash is a SHA1 hash of the current time in seconds since the Unix epoch.
// The second hash is a SHA256 hash of the same data.
pub fn generate_assorted_hashes() -> (String, String) {
    let unix_time = Utc::now().timestamp();
    (
        format!("{:x}", Sha1::digest(&unix_time.to_string().as_bytes())),
        format!("{:x}", Sha256::digest(&unix_time.to_string().as_bytes())),
    )
}
