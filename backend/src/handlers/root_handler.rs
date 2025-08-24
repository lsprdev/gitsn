use axum::{response::IntoResponse, http::StatusCode, Json};
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct Message {
    message: String,
}

#[utoipa::path(
    get,
    path = "/",
    responses(
        (status = 200, description = "Hello World message", body = Message)
    ),
    tag = "root",
    description = "This is the root endpoint"
)]
pub async fn root_handler() -> impl IntoResponse {
    let response = Message {
        message: "Hello, World!".to_string(),
    };

    (StatusCode::OK, Json(response))
}