use axum::{extract::Path, routing::get, Json, Router};
use serde_json::{json, Value};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/user/:id", get(get_user));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await.unwrap();
    println!("Serveur started on http://localhost:3001");
    axum::serve(listener, app).await.unwrap();
}

// PUT ONE IN COMMENT TO TEST

// compiles but forces us to handle error avoiding a server crash
/*async fn get_user(Path(id): Path<u32>) -> Json<Value> {
    let users = vec![
        json!({"id": 1, "name": "Alice"}),
        json!({"id": 2, "name": "Bob"}),
    ];

    match users.iter().find(|u| u["id"] == id) {
        Some(user) => Json(user.clone()),
        None => Json(json!({"error": "user inexistent"})),
    }
}   */

// similar to bug.js but won't compile compared to javascript
async fn get_user(Path(id): Path<u32>) -> Json<Value> {
    let users = vec![
        json!({"id": 1, "name": "Alice"}),
        json!({"id": 2, "name": "Bob"}),
    ];

    let user = users.iter().find(|u| u["id"] == id);

    // we try to use the user without taking care of none possibility
    Json(user.clone())
}   