mod auth;
mod utils;

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use auth::{check_credential, Credential};

async fn login(credential: Result<web::Form<Credential>, actix_web::Error>) -> impl Responder {
    match credential {
        Ok(credential) => check_credential(
            credential.username.clone(),
            credential.password.clone(),
        ),
        Err(_) => HttpResponse::BadRequest().body("Invalid credentials"),
    }
}

async fn check_token() -> impl Responder {
    HttpResponse::BadRequest().body("Invalid credentials")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/login", web::post().to(login))
            .route("/checktoken", web::get().to(check_token))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
