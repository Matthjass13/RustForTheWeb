use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    pub id: Option<i32>,
    pub name: String,
    pub email: String,
}

#[derive(Deserialize)]
pub struct NewUser {
    pub name: String,
    pub email: String,
}
