use axum::{
    http::{header, StatusCode},
    response::{Html, IntoResponse, Response},
    routing::{get, post},
    extract::State,
    Router,
};

use askama::Template;
use serde_json::to_string;

use crate::{handler::{enter_attempt_handler, enter_results_handler}, AppState};

pub fn create_router() -> Router {
    Router::new()
        .route("/", get(get_index))
        .route("/api/enter-attempt", post(enter_attempt_handler))
        .route("/api/enter-results", post(enter_results_handler))
        .with_state(AppState::default())
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate<'a> {
    results: &'a str,
}

impl std::fmt::Debug for IndexTemplate<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.results)
    }
}

impl IntoResponse for IndexTemplate<'_> {
    fn into_response(self) -> Response {
        match self.render() {
            Ok(html) => {
                let mut response = Html(html).into_response();
                response.headers_mut().insert(
                    header::CONTENT_TYPE,
                    header::HeaderValue::from_static("text/html"),
                );
                response
            }
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        }
    }
}

async fn get_index(State(app_state): State<AppState>) -> impl IntoResponse {
    let results_guard = app_state.results.lock().unwrap();

    let results_str = match to_string(&*results_guard) {
        Ok(json) => json,
        Err(_) => "Error formatting results".to_string(),
    };

    let template = IndexTemplate {
        results: &results_str,
    };

    template.into_response()
}