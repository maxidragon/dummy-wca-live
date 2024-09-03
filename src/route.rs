use axum::{
    routing::post,
    Router,
};

use crate::handler::{enter_attempt_handler, enter_results_handler};

pub fn create_router() -> Router {
    Router::new()
        .route("/api/enter-attempt", post(enter_attempt_handler))
        .route("/api/enter-results", post(enter_results_handler))
}