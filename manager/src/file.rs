use std::{fs, path::Path};

use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize, Deserialize, Debug)]
pub struct Cache {
    pub file_location: String,
}

pub(crate) fn get_passwords() -> serde_json::Value {
    let passwords = fs::read_to_string(fetch_cache().file_location).expect("Unable to read file");
    serde_json::from_str(&passwords).expect("Unable to parse json")
}

pub(crate) fn fetch_cache() -> Cache {
    let cache = fetch_cache_file();
    match cache {
        Ok(cache) => cache,
        Err(err) => {
            panic!("{}", err);
        }
    }
}

pub(crate) fn get_master_key() -> String {
    let cache_dir = format!("{}/.cache/spm", std::env::var("HOME").unwrap());
    let credentials_path = format!("{}/credentials.spm", &cache_dir);

    let credentials_file = fs::read_to_string(&credentials_path).unwrap();
    let credentials: serde_json::Value = serde_json::from_str(&credentials_file).unwrap();

    credentials["key"].as_str().unwrap().to_owned()
}

fn fetch_cache_file() -> Result<Cache, serde_json::Error> {
    let cache_file = format!("{}/.cache/spm/config.json", std::env::var("HOME").unwrap());

    let result = match fs::read_to_string(&cache_file) {
        Ok(it) => it,
        Err(_) => {
            let cache_dir = format!("{}/.cache/spm", std::env::var("HOME").unwrap());
            let pass_dir = format!("{}/.local/share/spm", std::env::var("HOME").unwrap());

            if !Path::new(&cache_dir).exists() {
                fs::create_dir_all(cache_dir).unwrap();
            }

            let base_data = json!({
                "file_location": format!("{}/passwords.spm", pass_dir),
            });

            let password_file_base_data = json!({
                "passwords": []
            });

            if !Path::new(&pass_dir).exists() {
                fs::create_dir_all(pass_dir).unwrap();
            }

            fs::write(cache_file, &base_data.to_string()).unwrap();
            fs::write(
                &base_data["file_location"].as_str().unwrap().to_string(),
                &password_file_base_data.to_string(),
            )
            .unwrap();
            base_data.to_string()
        }
    };

    serde_json::from_str(&result)
}
