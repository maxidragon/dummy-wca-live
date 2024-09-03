use axum::{http::StatusCode, response::IntoResponse, Json};
use axum_extra::{headers::{self, authorization::Bearer}, TypedHeader};
use serde_json::json;

use crate::schema::{EnterAttemptOptions, EnterResultsOptions};

pub async fn enter_attempt_handler(
    bearer_token: TypedHeader<headers::Authorization<Bearer>>,
    Json(body): Json<EnterAttemptOptions>
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let token_str = bearer_token.token();
    if token_str != "wca-live-token" {
        return Err((
            StatusCode::UNAUTHORIZED,
            Json(json!({"status": "error","message": "Unauthorized"})),
        ))
    }
    println!("Entering attempt for competition {}, event {}, round {}, attemptNumber {}, registrantId {}, attemptResult {}",
        body.competition_wca_id, body.event_id, body.round_number, body.registrant_id, body.attempt_number, body.attempt_result);
    return Ok(Json(json!({"message": "Attempt entered"})));
}

pub async fn enter_results_handler(
    bearer_token: TypedHeader<headers::Authorization<Bearer>>,
    Json(body): Json<EnterResultsOptions>
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let token_str = bearer_token.token();
    if token_str != "wca-live-token" {
        return Err((
            StatusCode::UNAUTHORIZED,
            Json(json!({"status": "error","message": "Unauthorized"})),
        ))
    }
    println!("Entering results for competition {}, event {}, round {}, results {:?}",
        body.competition_wca_id, body.event_id, body.round_number, body.results);
    return Ok(Json(json!({"message": "Results entered"})));
}