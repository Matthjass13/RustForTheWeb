use argon2::{
    Argon2, PasswordHasher, PasswordVerifier,
    password_hash::{PasswordHash, SaltString, rand_core::OsRng},
};
use jsonwebtoken::{EncodingKey, Header, encode};
use rocket::serde::json::Json;
use rocket::{State, http::Status, post};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::DbConn;

#[derive(Serialize, Deserialize, PartialEq)]
pub enum Role {
    Admin,
    User,
}

#[derive(Deserialize)]
pub struct AuthPayload {
    pub email: String,
    pub password: String,
    pub role: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub user_id: i32,
    pub exp: usize,
    pub role: Role,
}

#[derive(Serialize)]
pub struct AuthResponse {
    pub token: String,
}

#[post("/register", data = "<payload>")]
pub async fn register(
    db: &State<DbConn>,
    payload: Json<AuthPayload>,
) -> Result<Json<AuthResponse>, Status> {
    let salt = SaltString::generate(&mut OsRng);
    let password_hash = Argon2::default()
        .hash_password(payload.password.as_bytes(), &salt)
        .map_err(|_| Status::InternalServerError)?
        .to_string();

    sqlx::query("INSERT INTO rocket_accounts (email, password_hash, role) VALUES ($1, $2, $3)")
        .bind(&payload.email)
        .bind(&password_hash)
        .bind(payload.role.as_deref().unwrap_or("user"))
        .execute(&db.pool)
        .await
        .map_err(|_| Status::InternalServerError)?;

    Ok(Json(AuthResponse {
        token: "Account created".to_string(),
    }))
}

#[post("/login", data = "<payload>")]
pub async fn login(
    db: &State<DbConn>,
    payload: Json<AuthPayload>,
) -> Result<Json<AuthResponse>, Status> {
    let account: (String, i32, String) =
        sqlx::query_as("SELECT password_hash, id, role FROM rocket_accounts WHERE email = $1")
            .bind(&payload.email)
            .fetch_one(&db.pool)
            .await
            .map_err(|_| Status::Unauthorized)?;

    let parsed_hash = PasswordHash::new(&account.0).map_err(|_| Status::InternalServerError)?;

    Argon2::default()
        .verify_password(payload.password.as_bytes(), &parsed_hash)
        .map_err(|_| Status::Unauthorized)?;

    let exp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as usize
        + 3600;

    let role = match account.2.as_str() {
        "admin" => Role::Admin,
        _ => Role::User,
    };

    let claims = Claims {
        user_id: account.1,
        exp,
        role,
    };

    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET not defined");

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(|_| Status::InternalServerError)?;

    Ok(Json(AuthResponse { token }))
}
