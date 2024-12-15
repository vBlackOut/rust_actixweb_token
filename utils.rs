use std::collections::HashMap;
use actix_web::HttpResponse;
use chrono::{Local, NaiveDateTime, Duration};
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use crate::auth::AuthData;

pub fn find_expired_tokens(
    username: &str,
    password: &str,
    tokens: &HashMap<String, AuthData>,
) -> Result<Vec<String>, String> {
    let mut expired_tokens = Vec::new();

    for (token_key, auth_data) in tokens {
        let AuthData::Single(credential) = auth_data;

        if credential.username == username && credential.password == password {
            let is_expired = if let Ok(expire_date) = NaiveDateTime::parse_from_str(
                &credential.expire,
                "%d-%m-%Y %H:%M:%S",
            ) {
                Local::now().naive_local() >= expire_date
            } else {
                false
            };

            if is_expired {
                expired_tokens.push(token_key.clone());
            } else {
                return Err("Your token is available.".to_string());
            }
        }
    }

    if expired_tokens.is_empty() {
        Err("Invalid credentials".to_string())
    } else {
        Ok(expired_tokens)
    }
}

pub fn update_expired_tokens(
    tokens: &mut HashMap<String, AuthData>,
    expired_tokens: Vec<String>,
) -> HttpResponse {

    for token_key in expired_tokens {
        if let Some(auth_data) = tokens.get_mut(&token_key) {
            let AuthData::Single(credential) = auth_data;
            let expire_date = Local::now().naive_local() + Duration::hours(5);
            let expire_date = expire_date.format("%d-%m-%Y %H:%M:%S").to_string();
            credential.expire = expire_date.clone();
            println!("Updated token {} to new expire date {}", token_key, expire_date);
        }
    }

    // reecrire le fichier json
    rewrite_expired_dates(tokens);

    HttpResponse::Ok().body("Token updated successfully")
}

// Charger les tokens depuis un fichier JSON
pub fn load_tokens() -> HashMap<String, AuthData> {
    let mut contents = String::new();
    let mut file = File::open("config_auth.json").expect("Failed to open file");
    file.read_to_string(&mut contents).expect("Failed to read file");

    serde_json::from_str(&contents).expect("Invalid JSON format")
}

// Sauvegarder les tokens dans un fichier JSON
pub fn rewrite_expired_dates(tokens: &mut HashMap<String, AuthData>) {
    let serialized = serde_json::to_string_pretty(tokens).expect("Failed to serialize tokens");
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open("config_auth.json")
        .expect("Failed to open file for writing");

    file.write_all(serialized.as_bytes())
        .expect("Failed to write to file");
}
