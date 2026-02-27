mod model;
use rocket::serde::json::Json;
use rocket::{tokio::task::id, *};

use crate::model::User;

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, get_users, get_one_user])
}

#[get("/")]
fn index() -> &'static str {
    "
    USAGE

      POST /

          accepts raw data in the body of the request and responds with a URL of
          a page containing the body's content

      GET /<id>

          retrieves the content for the paste with id `<id>`
    "
}

#[get("/get_users")]
fn get_users() -> Json<Vec<User>> {
    let users: Vec<User> = vec![
        User {
            id: 1,
            name: "Jane".to_string(),
            email: "ttt".to_string(),
        },
        User {
            id: 2,
            name: "John".to_string(),
            email: "ddd".to_string(),
        },
    ];
    Json(users)
}

#[get("/get_users/<user_id>")]
fn get_one_user(user_id: i32) -> Json<User> {
    let user: User = User {
        id: user_id,
        name: "Jane".to_string(),
        email: "ttt".to_string(),
    };
    Json(user)
}
