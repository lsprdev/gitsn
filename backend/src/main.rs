use axum::{
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use backend::{
    handlers::{
        root_handler::{root_handler, Message},
        user_handler::create_user,
    },
    models::user::{CreateUser, User},
};

#[allow(dead_code)]
#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", get(root_handler))
        .route("/users", post(create_user))
        .merge(SwaggerUi::new("/swagger").url("/api-doc/openapi.json", ApiDoc::openapi()));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
        
    tracing::info!("Listening on http://{}", listener.local_addr().unwrap());
    tracing::info!("API documentation available at http://{}/swagger", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

#[derive(OpenApi)]
#[openapi(
    info(
        title = "GitSN API",
        description = "This is the API documentation of GitSN"
    ),
    paths(
        backend::handlers::root_handler::root_handler,
        backend::handlers::user_handler::create_user
    ),
    components(
        schemas(User, CreateUser, Message)
    )
)]
pub struct ApiDoc;