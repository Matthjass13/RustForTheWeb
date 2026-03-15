mod model;
use rocket::fs::FileServer;
use rocket::serde::json::Json;
use rocket::{State, http::Status, response::status::Custom};
use rocket::{tokio::task::id, *};
use rocket_cors::{AllowedOrigins, CorsOptions};
use sqlx::postgres::PgPoolOptions;
use std::env;

use crate::model::{NewUser, User};

struct DbConn {
    pool: sqlx::PgPool,
}

#[launch]
async fn rocket() -> _ {
    match dotenv::dotenv() {
        Ok(path) => println!("✅ .env chargé depuis : {:?}", path),
        Err(e) => println!("❌ .env non trouvé : {}", e),
    }

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL is not defined");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Impossible de se connecter à la DB");

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS users (
                    id SERIAL PRIMARY KEY,
                    name VARCHAR(100) NOT NULL,
                    email VARCHAR(100) NOT NULL)",
    )
    .execute(&pool)
    .await
    .expect("Impossible to create database");

    let cors = CorsOptions::default()
        .allowed_origins(AllowedOrigins::all())
        .to_cors()
        .expect("Error while building CORS");

    rocket::build()
        .manage(DbConn { pool })
        .mount(
            "/api",
            routes![
                index,
                get_users,
                get_one_user,
                add_user,
                update_user,
                delete_user
            ],
        )
        .mount("/", FileServer::from("static/"))
        .attach(cors)
}

#[get("/")]
async fn index(db: &State<DbConn>) -> String {
    let row: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM information_schema.tables")
        .fetch_one(&db.pool)
        .await
        .unwrap();

    format!("Nombre de tables: {}", row.0)
}

#[get("/get_users")]
async fn get_users(db: &State<DbConn>) -> Result<Json<Vec<User>>, String> {
    let users = sqlx::query_as("SELECT id, name, email FROM users")
        .fetch_all(&db.pool)
        .await
        .map_err(|e: sqlx::Error| e.to_string())?;

    Ok(Json(users))
}

#[get("/get_users/<user_id>")]
async fn get_one_user(user_id: i32, db: &State<DbConn>) -> Result<Json<User>, String> {
    let user = sqlx::query_as("Select id, name, email FROM users WHERE id = $1")
        .bind(user_id)
        .fetch_one(&db.pool)
        .await
        .map_err(|e: sqlx::Error| e.to_string())?;

    Ok(Json(user))
}

#[post("/add_user", data = "<user>")]
async fn add_user(db: &State<DbConn>, user: Json<NewUser>) -> Result<Json<User>, String> {
    let newUser = sqlx::query_as(
        "
        INSERT INTO users (name, email)
        VALUES ($1, $2)
        RETURNING id, name, email
        ",
    )
    .bind(&user.name)
    .bind(&user.email)
    .fetch_one(&db.pool)
    .await
    .map_err(|e: sqlx::Error| e.to_string())?;

    Ok(Json(newUser))
}

#[put("/update_user/<id>", data = "<user>")]
async fn update_user(
    db: &State<DbConn>,
    id: i32,
    user: Json<NewUser>,
) -> Result<Json<User>, String> {
    let updated_user = sqlx::query_as(
        "
        UPDATE users
        SET name = $1, email = $2
        WHERE id = $3
        RETURNING *
        ",
    )
    .bind(&user.name)
    .bind(&user.email)
    .bind(id)
    .fetch_one(&db.pool)
    .await
    .map_err(|e: sqlx::Error| e.to_string())?;

    Ok(Json(updated_user))
}

#[delete("/delete_user/<id>")]
async fn delete_user(db: &State<DbConn>, id: i32) -> Result<Status, String> {
    let Dbstatus = sqlx::query("DELETE FROM users WHERE id = $1")
        .bind(id)
        .execute(&db.pool)
        .await
        .map_err(|e: sqlx::Error| e.to_string())?;

    if Dbstatus.rows_affected() == 0 {
        return Ok(Status::NotFound);
    }

    Ok(Status::NoContent)
}
