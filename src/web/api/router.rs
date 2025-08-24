use crate::middlewares::not_found_mw::not_found_middleware;
use crate::middlewares::request_id_mw::RequestIdLayer;
use crate::middlewares::request_logging_mw::RequestLoggingLayer;
use crate::middlewares::timeout_mw::TimeoutLayer;
use crate::web::api::app_state::AppState;
use crate::web::api::v1::register_v1_routers;
use axum::Router;
use http::Method;
use std::time::Duration;
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
        .layer(RequestIdLayer::default())
        .layer(TimeoutLayer::new(Duration::from_secs(2)))
        .layer(RequestLoggingLayer::default())
        .fallback(not_found_middleware);

    app
}
