use crate::file::{get_master_key, get_passwords};
use chacha20poly1305::{
    aead::{Aead, NewAead},
    ChaCha20Poly1305, Key, Nonce,
};

pub fn get_password(name: String) {
    let passwords = get_passwords();

    if passwords["passwords"].as_array().is_none() {
        println!("No passwords found");
        return;
    }

    let password_data = passwords["passwords"]
        .as_array()
        .unwrap()
        .iter()
        .find(|x| x["name"].as_str().unwrap() == name);

    if password_data.is_none() {
        println!("Password not found");
        return;
    }

    let password = &password_data.unwrap()["password"]
        .as_array()
        .unwrap()
        .to_vec();
    let nonce = &password_data.unwrap()["nonce"].as_array().unwrap().to_vec();

    // Map Vec<Value> to Vec<u8>
    let password = password
        .iter()
        .map(|x| x.as_u64().unwrap() as u8)
        .collect::<Vec<u8>>();

    let nonce = nonce
        .iter()
        .map(|x| x.as_u64().unwrap() as u8)
        .collect::<Vec<u8>>();

    let master_key = get_master_key();

    let key = Key::from_slice(master_key.as_bytes());
    let nonce = Nonce::from_slice(&nonce);

    let cipher = ChaCha20Poly1305::new(key);

    let cipher_password = cipher
        .decrypt(nonce, password.as_ref())
        .expect("Failed to decrypt");

    print!("{}", String::from_utf8(cipher_password).unwrap());
}
