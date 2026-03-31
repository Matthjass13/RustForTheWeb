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
        _ctx: &AppContext,
    ) -> Result<Router> {


        
        let cors = CorsLayer::new()
            .allow_origin(Any)

            // Here, we allow all http methods so we can make the CRUD app work fully.
            
            .allow_methods([
                Method::GET,
                Method::POST,
                Method::PUT,
                Method::DELETE,
                Method::PATCH,
                Method::OPTIONS,
            ])
            .allow_headers(Any);

        Ok(router.layer(cors))
    }
}