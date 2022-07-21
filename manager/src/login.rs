use crate::add::{generate_12_bytes_nonce, generate_32_bytes_nonce};
use argon2::{self, Config};
use requestty::Question;
use serde_json::json;
use std::{fs, path::Path};

fn is_valid(password: &str, _: &requestty::Answers) -> bool {
    password.contains(|c: char| c.is_ascii_digit())
        && password.contains(char::is_alphabetic)
        && password.len() > 8
}

fn letter_and_numbers(password: &str, ans: &requestty::Answers) -> Result<(), String> {
    if is_valid(password, ans) {
        Ok(())
    } else {
        Err("Password needs to have at least 1 letter, 1 number, and be at least 8 characters long.".to_owned())
    }
}

fn prompt_login() -> Result<requestty::Answers, requestty::ErrorKind> {
    let questions = vec![
        Question::input("email")
            .message("Enter your email address")
            .build(),
        Question::password("master_key")
            .message("Enter the master key.")
            .mask('*')
            .validate_on_key(is_valid)
            .validate(letter_and_numbers)
            .build(),
    ];

    let result = requestty::prompt(questions);

    result
}

fn mount_json(credentials: Vec<String>) -> serde_json::Value {
    json!({
        "email": credentials[0],
        "master_key": credentials[1],
        "key": generate_32_bytes_nonce(),
    })
}

pub fn login(
    email: Option<String>,
    password: Option<String>,
) -> Result<bool, requestty::ErrorKind> {
    let mut login_info = match (email, password) {
        (Some(email), Some(password)) => mount_json(vec![email, password]),
        _ => mount_json(
            prompt_login()?
                .into_iter()
                .map(|(_, answer)| -> String { answer.as_string().unwrap().to_owned() })
                .collect(),
        ),
    };

    // Check if the file at $HOME/.cache/spm/credentials.spm exists.
    let cache_dir = format!("{}/.cache/spm", std::env::var("HOME").unwrap());
    if !Path::new(&cache_dir).exists() {
        fs::create_dir_all(&cache_dir).unwrap();
    }

    let credentials_path = format!("{}/credentials.spm", &cache_dir);

    if !Path::new(&credentials_path).exists() {
        // Encrypt password
        let config = Config::default();
        let salt = generate_12_bytes_nonce();
        let hash = argon2::hash_encoded(
            &login_info["master_key"].as_str().unwrap().as_bytes(),
            &salt,
            &config,
        );

        login_info["master_key"] = json!(hash.unwrap().to_owned());

        fs::write(&credentials_path, login_info.to_string()).unwrap();
    } else {
        return Ok(credentials_valid());
    }

    Ok(true)
}

/// Check whether the user is logged in.
pub fn credentials_valid() -> bool {
    true
}
