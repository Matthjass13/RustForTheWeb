use async_trait::async_trait;
use loco_rs::app::{AppContext, Initializer};
use loco_rs::Result;
use tower_http::cors::{Any, CorsLayer};
use axum::{http::Method, Router};

pub struct CorsInitializer;

#[async_trait]
impl Initializer for CorsInitializer {

    fn name(&self) -> String {
        "cors".to_string()
    }

    async fn after_routes(
        &self, 
        router:Router,
        ctx: &AppContext,
    ) -> Result<Router> {

        let cors = CorsLayer::new()
            .allow_origin(Any)
            .allow_methods([
                Method::GET,
                Method::POST,
                Method::PUT,
                Method::DELETE,
                Method::OPTIONS,
            ])
            .allow_headers(Any);

        //ctx.router.lock().unwrap().layer(cors);

        Ok(router.layer(cors))
    }
}