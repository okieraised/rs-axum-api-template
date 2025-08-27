use crate::common::api_response::{Response, write_problem_json};
use crate::middlewares::not_found_mw::not_found_middleware;
use crate::middlewares::recovery_mw::RecoveryLayer;
use crate::middlewares::request_id_mw::{
    RequestIdLayer, request_id_from_headers,
};
use crate::middlewares::request_logging_mw::RequestLoggingLayer;
use crate::middlewares::timeout_mw::TimeoutLayer;
use crate::web::api::app_state::AppState;
use crate::web::api::v1::register_v1_routers;
use axum::Router;
use http::{Method, StatusCode};
use std::time::Duration;
use tokio::time;
use tower_http::cors::{Any, CorsLayer};

pub fn register_routers(state: AppState) -> Router {
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_origin(Any);

    let v1_router =
        Router::new().nest("/api/v1", register_v1_routers(state.clone()));

    let app = Router::new()
        .merge(v1_router)
        .layer(cors)
        .layer(TimeoutLayer::new(Duration::from_secs(2)))
        .layer(RequestLoggingLayer::default())
        .layer(RecoveryLayer::default())
        .layer(RequestIdLayer::default())
        .fallback(not_found_middleware);

    app
}

async fn ok_handler(
    mut headers: http::HeaderMap,
) -> impl axum::response::IntoResponse {
    let req_id = request_id_from_headers(&mut headers);

    Response::<serde_json::Value>::new_with_request_id(req_id)
        .populate(
            "OK",
            "All good",
            serde_json::json!({"hello":"world"}),
            None,
            None,
        )
        .with_status(StatusCode::CREATED)
}

async fn err_handler() -> impl axum::response::IntoResponse {
    write_problem_json(
        StatusCode::BAD_REQUEST,
        "https://example.com/problems/validation",
        "Invalid input",
        "The 'name' field is required.",
        "/err",
    )
}

async fn timeout_handler(
    mut headers: http::HeaderMap,
) -> impl axum::response::IntoResponse {
    let req_id = request_id_from_headers(&mut headers);

    time::sleep(Duration::from_secs(60)).await;

    Response::<serde_json::Value>::new_with_request_id(req_id)
        .populate(
            "OK",
            "All good",
            serde_json::json!({"hello":"world"}),
            None,
            None,
        )
        .with_status(StatusCode::CREATED)
}
