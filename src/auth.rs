use actix_web::HttpResponse;
use serde::{Deserialize, Serialize};
use crate::utils::{load_tokens, update_expired_tokens, find_expired_tokens};

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum AuthData {
    Single(AuthJson),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthJson {
    pub username: String,
    pub password: String,
    pub expire: String,
}

#[derive(Deserialize)]
pub struct Credential {
    pub username: String,
    pub password: String,
}

pub fn check_credential(username: String, password: String) -> HttpResponse {
    let mut tokens = load_tokens();

    let expired_tokens = match find_expired_tokens(&username, &password, &tokens) {
        Ok(tokens) => tokens,
        Err(err) => return HttpResponse::Unauthorized().body(err),
    };

    update_expired_tokens(&mut tokens, expired_tokens)
}
