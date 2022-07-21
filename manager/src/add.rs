use std::fs;

use crate::{configure::fetch_cache, file::get_master_key};
use chacha20poly1305::{
    aead::{Aead, NewAead},
    ChaCha20Poly1305, Key, Nonce,
};
use rand::{distributions::Alphanumeric, thread_rng, Rng, RngCore};

pub(crate) fn generate_12_bytes_nonce() -> [u8; 12] {
    let mut nonce: [u8; 12] = [0; 12];
    thread_rng().fill_bytes(&mut nonce);
    nonce
}

// TODO: Change this function name and location
pub(crate) fn generate_32_bytes_nonce() -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(32)
        .map(char::from)
        .collect()
}

/// Add the password function
pub fn add_password(
    name: String,
    password: Option<String>,
    email: Option<String>,
    url: Option<String>,
    notes: Option<String>,
) {
    let master_key = get_master_key();

    let key = Key::from_slice(master_key.as_bytes());
    let generate_bytes = generate_12_bytes_nonce();
    let nonce = Nonce::from_slice(&generate_bytes);

    let cipher = ChaCha20Poly1305::new(key);

    let cipher_password = cipher
        .encrypt(nonce, password.unwrap_or(generate_password(8)).as_bytes())
        .expect("Failed to encrypt");

    let cache = fetch_cache();
    // Fetch content of the cache file
    let content = fs::read_to_string(&cache.file_location).expect("Failed to read cache file");
    // Parse content json
    let mut content: serde_json::Value =
        serde_json::from_str(&content).expect("Failed to parse json");
    // Add new entry to the json
    content["passwords"]
        .as_array_mut()
        .unwrap()
        .push(serde_json::json!({
            "name": name,
            "password": &cipher_password,
            "email": email.unwrap_or("".to_string()),
            "url": url.unwrap_or("".to_string()),
            "notes": notes.unwrap_or("".to_string()),
            "nonce": &nonce.to_vec()
        }));

    // Write the new json to the cache file
    fs::write(
        cache.file_location,
        serde_json::to_string(&content).unwrap(),
    )
    .expect("Failed to write to cache file");
}

/// Generate a random password of the specified size using random bytes
/// and convert to a string
pub fn generate_password(size: u32) -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(size.try_into().unwrap())
        .map(char::from)
        .collect()
}
