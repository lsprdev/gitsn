use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::{
    handlers::{
        root_handler::{root_handler, Message},
        user_handler::create_user,
    },
    models::user::{CreateUser, User},
};

use axum::{
    http::{
        header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
        HeaderValue, Method,
    },
    routing::{get, post},
    Router,
};

use tower_http::cors::CorsLayer;

pub fn route_builder() -> Router {
    let cors = CorsLayer::new()
        .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_credentials(true)
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

    Router::new()
        .route("/", get(root_handler))
        .route("/users", post(create_user))
        .merge(SwaggerUi::new("/swagger")
            .url("/api-doc/openapi.json", ApiDoc::openapi())).layer(cors)
}

#[derive(OpenApi)]
#[openapi(
    info(
        title = "GitSN API",
        description = "This is the API documentation of GitSN"
    ),
    paths(
        crate::handlers::root_handler::root_handler,
        crate::handlers::user_handler::create_user
    ),
    components(
        schemas(User, CreateUser, Message)
    )
)]
pub struct ApiDoc;