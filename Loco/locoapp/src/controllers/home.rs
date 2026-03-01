use loco_rs::prelude::*;

// _ctx contains your database connection, as well as other app resource that you'll need
async fn hello(State(_ctx): State<AppContext>) -> Result<Response> {
    format::text("ola, mundo")
}

pub fn routes() -> Routes {
    Routes::new().prefix("home").add("/hello", get(hello))
}