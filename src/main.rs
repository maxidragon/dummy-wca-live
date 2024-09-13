mod handler;
mod route;
mod schema;

use axum::http::{header::{AUTHORIZATION, CONTENT_TYPE}, Method};

use schema::ResultStruct;
use tokio::net::TcpListener;

use tower_http::cors::{Any, CorsLayer};

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Debug, Default, Clone)]
pub struct AppState {
    pub results: Arc<Mutex<HashMap<String, HashMap<i32, Vec<ResultStruct>>>>>,
}

#[tokio::main]
async fn main() {
    let port = "4000";
    let app_state = {};

    let app = route::create_router() // Fixed import and function
    .layer(
        CorsLayer::new()
            .allow_origin(Any)
            .allow_headers([AUTHORIZATION, CONTENT_TYPE])
            .allow_methods([Method::GET, Method::POST])
    )
    .with_state(app_state);


    println!("Server started successfully on port {}", port);

    let listener = TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}