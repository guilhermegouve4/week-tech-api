use axum::{extract::Json, response::IntoResponse, response::Response};
use axum::http::StatusCode;
use serde_json::json;
use crate::models::{claims::Claims, credential::Credential};
use jsonwebtoken::{encode, Header, EncodingKey};
use std::env::var;
use argon2::{
    Argon2,
    PasswordHash,
    PasswordVerifier,
};

pub async fn admin_login(Json(body): Json<Credential>) -> Response {
    let admin_email =
        var("ADMIN_EMAIL").expect("Could not read ADMIN_EMAIL from env");

    let admin_password_hash =
        var("ADMIN_PASSWORD_HASH").expect("Could not read ADMIN_PASSWORD_HASH from env");

    if body.email != admin_email {
        return (StatusCode::UNAUTHORIZED, Json(json!({"error": "invalid_credentials"}))).into_response();
    }

    let parsed_hash =
        PasswordHash::new(&admin_password_hash).expect("Invalid hash");

    let is_correct: bool =
        Argon2::default().verify_password(body.password.as_bytes(), &parsed_hash).is_ok();

    if !is_correct {return (StatusCode::UNAUTHORIZED, Json(json!({"error": "invalid_credentials"}))).into_response();}

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

    (StatusCode::OK, Json(json!({ "token": token }))).into_response()
}
