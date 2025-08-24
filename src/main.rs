mod applications;
mod common;
mod config;
mod constants;
mod domains;
mod infrastructures;
mod middlewares;
mod services;
mod web;

use crate::common::api_response;
use crate::common::api_response::{Response, write_problem_json};
use crate::infrastructures::log::logger::setup_logger;
use crate::infrastructures::otel::tracer::init_tracer_provider;
use crate::middlewares::not_found_mw::not_found_middleware;
use crate::middlewares::recovery_mw::RecoveryLayer;
use crate::middlewares::request_id_mw::{
    RequestIdLayer, request_id_from_headers,
};
use crate::middlewares::request_logging_mw::RequestLoggingLayer;
use crate::middlewares::timeout_mw::TimeoutLayer;
use axum::{Router, http::StatusCode, routing::get};
use log::info;
use std::time::Duration;
use tokio::{signal, time};

#[tokio::main]
async fn main() {
    setup_logger();
    let provider = init_tracer_provider().unwrap();
    opentelemetry::global::set_tracer_provider(provider);

    info!("start");

    let app = Router::new()
        .route("/ok", get(ok_handler))
        .route("/err", get(err_handler))
        .route("/timeout", get(timeout_handler))
        .layer(TimeoutLayer::new(Duration::from_secs(2)))
        .layer(RequestLoggingLayer::default())
        .layer(RecoveryLayer::default())
        .layer(RequestIdLayer::default())
        .fallback(not_found_middleware);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8880").await.unwrap();
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
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

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(unix)]
    let interrupt = async {
        signal::unix::signal(signal::unix::SignalKind::interrupt())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();
    #[cfg(not(unix))]
    let interrupt = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
        _ = interrupt => {},
    }
}
