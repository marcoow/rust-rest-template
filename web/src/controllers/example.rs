use crate::{internal_error, state::AppState};
use axum::{extract::Path, extract::State, http::StatusCode, Json, response::Html};
use serde::Deserialize;
#[cfg(test)]
use serde::Serialize;
use tracing::info;
use uuid::Uuid;
use validator::Validate;

pub async fn hello() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}
