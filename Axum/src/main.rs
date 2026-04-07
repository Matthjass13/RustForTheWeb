mod models;

use axum::{
    extract::{Path, State},
    http::{header, Request, StatusCode}, // Added header and Request
    middleware::{self, Next},           // Added for custom auth
    response::{IntoResponse, Response},  // Added Response
    routing::{delete, get, post, put},
    Json, Router,
};
use models::{create_user, delete_user, get_user, update_user_email, User};
use redis::{Client as RedisClient, Commands};
use serde::Deserialize;
use serde_json::json;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::env;
use std::sync::Arc;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH}; // Added Duration
use tower::ServiceBuilder;                                 // Added for middleware management
use tower_http::{trace::TraceLayer, timeout::TimeoutLayer}; // Added standard tower layers
use tower_http::cors::{Any, CorsLayer};

#[derive(Deserialize)]
struct CreateUserRequest {
    name: String,
    email: String,
}

#[derive(Deserialize)]
struct UpdateEmailRequest {
    email: String,
}

struct SharedState {
    pool: PgPool,
    redis: RedisClient,
    django_url: String,
}

type AppState = Arc<SharedState>;

#[tokio::main]
async fn main() {
    // 1. Setup Environment and Connections
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let redis_url = env::var("REDIS_URL").unwrap_or_else(|_| "redis://redis:6379".to_string());
    let django_url = env::var("DJANGO_URL").unwrap_or_else(|_| "http://python_be:8000".to_string());

    // Initialize Tracing (crucial for TraceLayer to work!)
    tracing_subscriber::fmt::init();

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create pool.");

    let redis_client = RedisClient::open(redis_url).expect("Invalid Redis URL");

    let state: AppState = Arc::new(SharedState {
        pool,
        redis: redis_client,
        django_url,
    });

// 1. Define the middleware stack for PROTECTED routes

    let cors = CorsLayer::new()
        .allow_origin(Any) 
        .allow_methods(Any)
        .allow_headers(Any);

    let api_middleware = ServiceBuilder::new()
        .layer(TraceLayer::new_for_http())
        .layer(cors)
        .layer(TimeoutLayer::new(Duration::from_secs(30)))
        .layer(middleware::from_fn(auth_middleware));

    // 2. Create the API router and apply the auth layer to it
    let api_routes = Router::new()
        .route("/users", post(handle_create_user))
        .route("/users/:id", get(handle_get_user))
        .route("/users/:id", put(handle_update_email))
        .route("/users/:id", delete(handle_delete_user))
        .route("/rust/sort", get(handle_rust_sort))
        .route("/django/sort", get(handle_django_proxy))
        .route("/race-status", get(handle_race_status))
        .layer(api_middleware); // Only these routes require the token

    // 3. Create the main app and merge the public + private routes
    let app = Router::new()
        .route("/", get(index)) // This is public!
        .merge(api_routes)      // Add the protected routes
        .with_state(state);     // State is shared across everything

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Listening on http://0.0.0.0:3000 (Index is public, API is private)");
    axum::serve(listener, app).await.unwrap();
}

// --- Custom Authentication Middleware ---

async fn auth_middleware(
    req: Request<axum::body::Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    // Extract Authorization header
    let auth_header = req.headers()
        .get(header::AUTHORIZATION)
        .and_then(|h| h.to_str().ok());

    // Basic Token Check (In a PoC, this might be a static env var)
    // For production, you'd verify a JWT or session here.
    const AUTH_TOKEN: &str = "Bearer secret-poc-token";

    if let Some(token) = auth_header {
        if token == AUTH_TOKEN {
            // Token is valid, proceed to the next layer/handler
            return Ok(next.run(req).await);
        }
    }

    // Token is missing or invalid
    Err(StatusCode::UNAUTHORIZED)
}

async fn index() -> axum::response::Html<&'static str> {
    axum::response::Html(include_str!("../static/index.html"))
}

// --- User Handlers ---

async fn handle_create_user(
    State(state): State<AppState>,
    Json(body): Json<CreateUserRequest>,
) -> StatusCode {
    match create_user(&state.pool, &body.name, &body.email).await {
        Ok(_) => StatusCode::CREATED,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

async fn handle_get_user(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<User>, StatusCode> {
    match get_user(&state.pool, id).await {
        Ok(user) => Ok(Json(user)),
        Err(sqlx::Error::RowNotFound) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn handle_update_email(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(body): Json<UpdateEmailRequest>,
) -> StatusCode {
    match update_user_email(&state.pool, id, &body.email).await {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

async fn handle_delete_user(State(state): State<AppState>, Path(id): Path<i32>) -> StatusCode {
    match delete_user(&state.pool, id).await {
        Ok(_) => StatusCode::NO_CONTENT,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

// --- Performance Race Handlers ---

async fn handle_rust_sort(State(state): State<AppState>) -> impl IntoResponse {
    let mut data: Vec<i32> = (0..100_000).rev().collect();

    let start = Instant::now();
    let start_unix_ms = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64;

    let mut con = state.redis.get_connection().expect("Redis connection failed");

    // Reset all race state
    let _: () = con.set("race_winner", "none").unwrap();
    let _: () = con.set("race_start_ms", start_unix_ms).unwrap();
    let _: () = con.set("rust_progress", 0).unwrap();
    let _: () = con.set("rust_swaps", 0u64).unwrap();
    let _: () = con.set("rust_comparisons", 0u64).unwrap();
    let _: () = con.set("django_progress", 0).unwrap();
    let _: () = con.set("django_swaps", 0u64).unwrap();
    let _: () = con.set("django_comparisons", 0u64).unwrap();

    let n = data.len();
    let mut total_swaps: u64 = 0;
    let mut total_comparisons: u64 = 0;

    for i in 0..n {
        // Flush telemetry to Redis every 1000 outer iterations
        if i % 1000 == 0 {
            let progress = (i as f32 / n as f32 * 100.0) as i32;
            let elapsed_ms = start.elapsed().as_millis() as u64;
            let _: () = con.set("rust_progress", progress).unwrap();
            let _: () = con.set("rust_swaps", total_swaps).unwrap();
            let _: () = con.set("rust_comparisons", total_comparisons).unwrap();
            let _: () = con.set("rust_elapsed_ms", elapsed_ms).unwrap();
        }

        for j in 0..n - i - 1 {
            total_comparisons += 1;
            if data[j] > data[j + 1] {
                data.swap(j, j + 1);
                total_swaps += 1;
            }
        }
    }

    let elapsed_ms = start.elapsed().as_millis() as u64;

    // Final flush
    let _: () = con.set("rust_progress", 100).unwrap();
    let _: () = con.set("rust_swaps", total_swaps).unwrap();
    let _: () = con.set("rust_comparisons", total_comparisons).unwrap();
    let _: () = con.set("rust_elapsed_ms", elapsed_ms).unwrap();
    let _: () = con.set("race_winner", "rust").unwrap();

    Json(json!({
        "engine": "rust",
        "time_ms": elapsed_ms,
        "swaps": total_swaps,
        "comparisons": total_comparisons,
    }))
}

async fn handle_django_proxy(State(state): State<AppState>) -> impl IntoResponse {
    let url = format!("{}/django/sort", state.django_url);
    let client = reqwest::Client::new();
    match client
        .get(&url)
        .header("Host", "localhost")
        .send()
        .await
    {
        Ok(resp) => {
            let status = axum::http::StatusCode::from_u16(resp.status().as_u16())
                .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
            let body = resp.text().await.unwrap_or_default();
            (status, body)
        }
        Err(e) => {
            eprintln!("Django proxy error: {}", e);
            (StatusCode::BAD_GATEWAY, "Django unreachable".to_string())
        }
    }
}

async fn handle_race_status(State(state): State<AppState>) -> impl IntoResponse {
    let mut con = state.redis.get_connection().expect("Redis connection failed");

    let rust_progress: i32 = con.get("rust_progress").unwrap_or(0);
    let rust_swaps: u64 = con.get("rust_swaps").unwrap_or(0);
    let rust_comparisons: u64 = con.get("rust_comparisons").unwrap_or(0);
    let rust_elapsed_ms: u64 = con.get("rust_elapsed_ms").unwrap_or(0);

    let django_progress: i32 = con.get("django_progress").unwrap_or(0);
    let django_swaps: u64 = con.get("django_swaps").unwrap_or(0);
    let django_comparisons: u64 = con.get("django_comparisons").unwrap_or(0);
    let django_elapsed_ms: u64 = con.get("django_elapsed_ms").unwrap_or(0);

    let winner: String = con.get("race_winner").unwrap_or_else(|_| "none".to_string());
    let start_ms: u64 = con.get("race_start_ms").unwrap_or(0);

    Json(json!({
        "rust": {
            "progress": rust_progress,
            "swaps": rust_swaps,
            "comparisons": rust_comparisons,
            "elapsed_ms": rust_elapsed_ms,
        },
        "django": {
            "progress": django_progress,
            "swaps": django_swaps,
            "comparisons": django_comparisons,
            "elapsed_ms": django_elapsed_ms,
        },
        "winner": winner,
        "start_ms": start_ms,
    }))
}