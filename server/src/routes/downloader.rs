use axum::{Json, Router, extract::Query, routing::get};
use serde_json::{Value, json};
use std::collections::HashMap;

use crate::error::{AppError, AppResult};
use crate::state::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/getAll", get(get_all))
        .route("/get", get(get_by_id))
        .route("/add", get(add))
        .route("/pause", get(pause))
        .route("/resume", get(resume))
        .route("/remove", get(remove))
}

async fn get_all() -> Json<Value> {
    Json(json!([]))
}

async fn get_by_id(Query(params): Query<HashMap<String, String>>) -> AppResult<Json<Value>> {
    match params.get("id") {
        Some(_) => Err(AppError::not_found("there is no download with this id")),
        None => Err(AppError::bad_request("missing id")),
    }
}

async fn add(Query(params): Query<HashMap<String, String>>) -> AppResult<Json<Value>> {
    match params.get("url") {
        Some(_) => Err(AppError::unsupported("HTTP file downloader")),
        None => Err(AppError::bad_request("missing url")),
    }
}

async fn pause() -> AppResult<Json<Value>> {
    Err(AppError::unsupported("HTTP file downloader"))
}

async fn resume() -> AppResult<Json<Value>> {
    Err(AppError::unsupported("HTTP file downloader"))
}

async fn remove() -> AppResult<Json<Value>> {
    Err(AppError::unsupported("HTTP file downloader"))
}
