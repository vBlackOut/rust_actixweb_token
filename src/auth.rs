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

// Add trait security for check len password
pub trait ValidatePassword {
    fn is_valid(&self) -> bool;
}

// if password is > 5 is true !
impl ValidatePassword for String {
    fn is_valid(&self) -> bool {
        self.len() > 5
    }
}

pub fn check_credential<T>(username: String, password: String) -> HttpResponse
where
    T: ValidatePassword, // Add define security password > 5
{

    if !password.is_valid() {
        return HttpResponse::Unauthorized().body("Password must be longer than 5 characters");
    }

    let mut tokens = load_tokens();

    let expired_tokens = match find_expired_tokens(&username, &password, &tokens) {
        Ok(tokens) => tokens,
        Err(err) => return HttpResponse::Unauthorized().body(err),
    };

    update_expired_tokens(&mut tokens, expired_tokens)
}
