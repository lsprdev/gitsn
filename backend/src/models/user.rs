use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Deserialize, ToSchema)]
pub struct CreateUser {
    pub username: String,
}

#[derive(Serialize, ToSchema)]
pub struct User {
    pub id: u64,
    pub username: String,
}
