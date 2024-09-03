mod handler;
mod route;
mod schema;

use axum::http::{header::{AUTHORIZATION, CONTENT_TYPE}, Method};

use tokio::net::TcpListener;

use route::create_router;
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() {
    let port = "4000";

    let app = create_router().layer(
        CorsLayer::new()
            .allow_origin(Any)
            .allow_headers([AUTHORIZATION, CONTENT_TYPE])
            .allow_methods([Method::GET, Method::POST]),
    );

    println!("Server started successfully on port {}", port);

    let listener = TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}