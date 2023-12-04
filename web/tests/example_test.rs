use axum::{
    body::Body,
    http::{self, Method},
};
use axum_on_rails::test::helpers::{request, teardown, TestContext};
use axum_on_rails_procs::test;
use hyper::StatusCode;
use rust_rest_db::entities::Task;
use serde::Serialize;
use serde_json::json;
use std::collections::HashMap;
use uuid::Uuid;

mod common;

type TasksList = Vec<Task>;

#[test]
async fn test_hello(_context: &TestContext) {
    let response = request(
        &context.app,
        "/example",
        HashMap::new(),
        Body::empty(),
        Method::GET,
    )
    .await;

    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(response.into_body(), "<h1>Hello, World!</h1>");
}
