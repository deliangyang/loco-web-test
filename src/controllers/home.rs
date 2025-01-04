use axum::debug_handler;
use loco_rs::prelude::*;
use sea_orm::ActiveModelTrait;

use crate::models::_entities::post::ActiveModel;
use crate::views::home::HomeResponse;

#[debug_handler]
async fn current() -> Result<Response> {
    format::json(HomeResponse::new("loco"))
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api")
        .add("/", get(current))
        .add("/post/create", post(create_post))
        .add("/redis/test", post(redis_test))
}

async fn create_post(State(ctx): State<AppContext>) -> Result<Response> {
    let mut model = ActiveModel {
       ..Default::default()
    };
    model.title = Set("v1".to_string());
    model.text = Set("text v1".to_string());
    let item = model.insert(&ctx.db).await?;
    format::json(item)
}

async fn redis_test(State(ctx): State<AppContext>) -> Result<Response> {
    let _: () = ctx.cache.insert("test", "value").await?;
    let value: Option<String> = ctx.cache.get("test").await?;
    format::json(value)
}
