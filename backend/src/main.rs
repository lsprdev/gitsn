// Imports =================

use axum::Router;
use backend::route::route_builder;

// Main function =================
#[tokio::main]
async fn main() {
    
    let app: Router = route_builder();
    
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
    .await
    .unwrap();
    
    tracing_subscriber::fmt::init();
    tracing::info!("Listening on http://{}", listener.local_addr().unwrap());
    tracing::info!("API documentation available at http://{}/swagger", listener.local_addr().unwrap());
    
    axum::serve(listener, app).await.unwrap();
}
