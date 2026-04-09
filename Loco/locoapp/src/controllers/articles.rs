#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use loco_rs::prelude::*;
use serde::{Deserialize, Serialize};
use ammonia::{Builder, clean};
use std::collections::HashSet;
use axum_extra::extract::CookieJar;
use axum_extra::extract::cookie::Cookie;

/*
In this file, we defined how CRUD operations
should be handled in the backend.
*/

use crate::services::article_analysis::{analyze_articles_parallel_with_timing, ArticleAnalysis};
use crate::services::article_analysis::analyze_articles_sequential;
use crate::models::_entities::{
    articles::{ActiveModel, Entity, Model},
    comments,
};
use crate::models::_entities::users;


fn clean_strict(input: &str) -> String {
    Builder::new()
        .tags(HashSet::new()) // No html tag authorized
        .clean(input)
        .to_string()
}


pub async fn comments(
    Path(id): Path<i32>,
    State(ctx): State<AppContext>,
) -> Result<Response> {
    let item = load_item(&ctx, id).await?;
    let comments = item.find_related(comments::Entity).all(&ctx.db).await?;
    format::json(comments)
}

#[derive(Serialize)]
pub struct AnalysisResponse {
    pub sequential_results: Vec<ArticleAnalysis>,
    pub parallel_results: Vec<ArticleAnalysis>,
    
    pub sequential_duration_ms: u128,
    pub parallel_total_ms: u128,
    pub parallel_spawn_ms: u128,
    pub parallel_execution_ms: u128,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Params {
    pub title: Option<String>,
    pub content: Option<String>,
}


    


impl Params {

    fn sanitized(&self) -> Self {
        Self {
            title: self.title.as_ref().map(|t| clean_strict(t)),
            content: self.content.as_ref().map(|c| clean_strict(c)),
        }
    }

    fn update(&self, item: &mut ActiveModel) {
        item.title = Set(self.title.clone());
        item.content = Set(self.content.clone());
    }
}

async fn load_item(ctx: &AppContext, id: i32) -> Result<Model> {
    let item = Entity::find_by_id(id).one(&ctx.db).await?;
    item.ok_or_else(|| Error::NotFound)
}

pub async fn list(State(ctx): State<AppContext>) -> Result<Response> {
    format::json(Entity::find().all(&ctx.db).await?)
}

fn validate_jwt(jar: &CookieJar, ctx: &AppContext) -> Result<()> {
    let token = jar
        .get("token")
        .ok_or_else(|| Error::Unauthorized("missing token".into()))?;

    let jwt_secret = ctx.config.get_jwt_config()?;

    loco_rs::auth::jwt::JWT::new(&jwt_secret.secret)
        .validate(token.value())
        .or_else(|_| unauthorized("invalid token"))?;

    Ok(())
}

pub async fn add(
    jar: CookieJar,
    State(ctx): State<AppContext>,
    Json(params): Json<Params>,
) -> Result<Response> {
    validate_jwt(&jar, &ctx)?;

    let mut item: ActiveModel = Default::default();
    let sanitized = params.sanitized();
    sanitized.update(&mut item);
    let item = item.insert(&ctx.db).await?;
    format::json(item)
}

pub async fn update(
    jar: CookieJar,
    Path(id): Path<i32>,
    State(ctx): State<AppContext>,
    Json(params): Json<Params>,
) -> Result<Response> {
    validate_jwt(&jar, &ctx)?;

    let item = load_item(&ctx, id).await?;  // Type = Model
    let mut item = item.into_active_model(); // Type = ActiveModel
    let sanitized = params.sanitized();
    sanitized.update(&mut item);
    let item = item.update(&ctx.db).await?; // Type = Model
    format::json(item)
}

pub async fn remove(
    jar: CookieJar,
    Path(id): Path<i32>,
    State(ctx): State<AppContext>,
) -> Result<Response> {
    validate_jwt(&jar, &ctx)?;

    load_item(&ctx, id).await?.delete(&ctx.db).await?;
    format::empty()
}

pub async fn get_one(Path(id): Path<i32>, State(ctx): State<AppContext>) -> Result<Response> {
    format::json(load_item(&ctx, id).await?)
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("api/articles")
        .add("/", get(list))
        .add("/", post(add))
        .add("/{id}", get(get_one))
        .add("/{id}", delete(remove))
        .add("/{id}", patch(update))
        .add("/{id}/comments", get(comments))
        .add("/analyze", get(analyze))
}


pub async fn analyze(State(ctx): State<AppContext>) -> Result<Response> {
    let articles = Entity::find().all(&ctx.db).await?;

    let data: Vec<(i32, String)> = articles
        .into_iter()
        .map(|a| (a.id, a.content.unwrap_or_default()))
        .collect();

    use std::time::Instant;

    // Sequential
    let start_seq = Instant::now();
    let sequential_results = analyze_articles_sequential(data.clone());
    let sequential_duration = start_seq.elapsed().as_millis();

    // Parallel
    let parallel = analyze_articles_parallel_with_timing(data);

    let parallel_total = parallel.spawn_time_ms + parallel.execution_time_ms;

    format::json(AnalysisResponse {
        parallel_results: parallel.results,
        sequential_results,
        parallel_total_ms: parallel_total,
        parallel_spawn_ms: parallel.spawn_time_ms,
        parallel_execution_ms: parallel.execution_time_ms,
        sequential_duration_ms: sequential_duration,
    })
}