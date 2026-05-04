use serde::Deserialize;
use axum::extract::Json;
use crate::models::claims::Claims;
use jsonwebtoken::{encode, Header, EncodingKey};
use std::env::var;
use argon2::{
    Argon2,
    PasswordHash,
    PasswordVerifier,
};
#[derive(Deserialize)]
pub struct Credential {
    email: String,
    password:  String
}

pub async fn admin_login(Json(body): Json<Credential>) -> String {
    let admin_email =
        var("ADMIN_EMAIL").expect("Could not read ADMIN_EMAIL from env");

    let admin_password_hash =
        var("ADMIN_PASSWORD_HASH").expect("Could not read ADMIN_PASSWORD_HASH from env");

    if body.email != admin_email {
        return "unauthorized".to_string();
    }

    let parsed_hash =
        PasswordHash::new(&admin_password_hash).expect("Invalid hash");

    let is_correct: bool =
        Argon2::default().verify_password(body.password.as_bytes(), &parsed_hash).is_ok();

    if !is_correct {return "unauthorized".to_string();}

    let header = Header::default();

    let exp = jsonwebtoken::get_current_timestamp() + 7200;

    let claims = Claims {
        name: "admin".to_string(),
        exp: exp
    };

    let secret = var("JWT_SECRET")
        .expect("Error while getting secret key from env");

    let secret_as_bytes = secret.as_bytes();

    let key = EncodingKey::from_secret(secret_as_bytes); 

    let token = encode(&header, &claims, &key).expect("Error while encoding JWT key");

    return token;
}
