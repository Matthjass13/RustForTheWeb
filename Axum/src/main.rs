mod models;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{get, post, put, delete},
    Json, Router,
};

use models::{create_user, get_user, update_user_email, delete_user, User};
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use serde::Deserialize;
use std::env;
use std::sync::Arc;

#[derive(Deserialize)]
struct CreateUserRequest {
    name: String,
    email: String,
}

#[derive(Deserialize)]
struct UpdateEmailRequest {
    email: String,
}

type AppState = Arc<PgPool>;
#[tokio::main]
async fn main() {

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create pool.");

    let state: AppState = Arc::new(pool);

    let app = Router::new()
        .route("/",             get(index))
        .route("/users",        post(handle_create_user))
        .route("/users/:id",    get(handle_get_user))
        .route("/users/:id",    put(handle_update_email))
        .route("/users/:id",    delete(handle_delete_user))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Listening on http://0.0.0.0:3000");
    axum::serve(listener, app).await.unwrap();
}


async fn index() -> axum::response::Html<&'static str> {
    axum::response::Html(include_str!("../static/index.html"))
}

async fn handle_create_user(
    State(pool): State<AppState>,
    Json(body): Json<CreateUserRequest>,
) -> StatusCode {
    match create_user(&pool, &body.name, &body.email).await {
        Ok(_) => StatusCode::CREATED,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

async fn handle_get_user(
    State(pool): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<User>, StatusCode> {
    match get_user(&pool, id).await {
        Ok(user) => Ok(Json(user)),
        Err(sqlx::Error::RowNotFound) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn handle_update_email(
    State(pool): State<AppState>,
    Path(id): Path<i32>,
    Json(body): Json<UpdateEmailRequest>,
) -> StatusCode {
    match update_user_email(&pool, id, &body.email).await {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

async fn handle_delete_user(
    State(pool): State<AppState>,
    Path(id): Path<i32>,
) -> StatusCode {
    match delete_user(&pool, id).await {
        Ok(_) => StatusCode::NO_CONTENT,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}