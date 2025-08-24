use axum::{http::StatusCode, response::IntoResponse, Json};
use crate::models::user::{CreateUser, User};

#[utoipa::path(
    post,
    path = "/users",
    request_body = CreateUser,
    responses(
        (status = 201, description = "User created successfully", body = User)
    ),
    tag = "user",
    description = "User management endpoints"
)]
pub async fn create_user(Json(payload): Json<CreateUser>) -> impl IntoResponse {
    let user = User {
        id: 1337,
        username: payload.username,
    };

    (StatusCode::CREATED, Json(user))
}
