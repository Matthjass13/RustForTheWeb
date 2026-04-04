use async_trait::async_trait;
use loco_rs::app::{AppContext, Initializer};
use loco_rs::Result;
use tower_http::cors::{Any, CorsLayer};
use axum::{http::Method, Router};
use axum::http::HeaderValue;

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
            .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
            .allow_credentials(true)
            .allow_methods([
                Method::GET,
                Method::POST,
                Method::PUT,
                Method::DELETE,
                Method::PATCH,
                Method::OPTIONS,
            ])
            .allow_headers([
                axum::http::header::CONTENT_TYPE,
                axum::http::header::AUTHORIZATION,
                axum::http::header::ACCEPT,
            ]);

        Ok(router.layer(cors))
    }
}