use crate::common::api_response::Response;
use crate::domains::health::HealthcheckTrait;
use crate::middlewares::request_id_mw::request_id_from_headers;
use axum::Router;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::routing::get;
use http::{HeaderMap, StatusCode};
use std::sync::Arc;

#[derive(Clone)]
pub struct HealthcheckDeps {
    pub healthcheck: Arc<dyn HealthcheckTrait>,
    // pub tracer: Tracer,
    // pub logger: Arc<Logger>,
}

pub fn new_healthcheck_router(state: HealthcheckDeps) -> Router {
    Router::new()
        .route("", get(health))
        .route("/live", get(live))
        .route("/ready", get(ready))
        .route("/started", get(started))
        .with_state(state)
}

pub async fn health(
    mut headers: HeaderMap, State(state): State<HealthcheckDeps>,
) -> impl IntoResponse {
    let req_id = request_id_from_headers(&mut headers);
    let result = state.healthcheck.health().await;

    Response::new_with_request_id(req_id)
        .with_code(result.code)
        .with_message(result.message)
        .with_data(result.data)
        .with_status(StatusCode::OK)
}

pub async fn live(
    mut headers: HeaderMap, State(state): State<HealthcheckDeps>,
) -> impl IntoResponse {
    let req_id = request_id_from_headers(&mut headers);
    let result = state.healthcheck.live().await;
    Response::new_with_request_id(req_id)
        .with_code(result.code)
        .with_message(result.message)
        .with_data(result.data)
        .with_status(StatusCode::OK)
}

pub async fn ready(
    mut headers: HeaderMap, State(state): State<HealthcheckDeps>,
) -> impl IntoResponse {
    let req_id = request_id_from_headers(&mut headers);
    let result = state.healthcheck.ready().await;
    Response::new_with_request_id(req_id)
        .with_code(result.code)
        .with_message(result.message)
        .with_data(result.data)
        .with_status(StatusCode::OK)
}

pub async fn started(
    mut headers: HeaderMap, State(state): State<HealthcheckDeps>,
) -> impl IntoResponse {
    let req_id = request_id_from_headers(&mut headers);
    let result = state.healthcheck.started().await;
    Response::new_with_request_id(req_id)
        .with_code(result.code)
        .with_message(result.message)
        .with_data(result.data)
        .with_status(StatusCode::OK)
}
