mod applications;
mod common;
mod config;
mod constants;
mod domains;
mod infrastructures;
mod middlewares;
mod services;
mod web;

use crate::domains::authentication::AuthenticationTrait;
use crate::domains::health::HealthcheckTrait;
use crate::infrastructures::cache::local_cache::CacheRegistry;
use crate::infrastructures::database;
use crate::infrastructures::database::{DbPool, init_database_connection};
use crate::infrastructures::log::logger::setup_logger;
use crate::infrastructures::otel::tracer::init_tracer_provider;
use crate::services::v1::authentication::AuthenticationService;
use crate::services::v1::healthcheck::HealthcheckService;
use crate::web::api::app_state::AppState;
use crate::web::api::router::register_routers;
use anyhow::Error;
use log::info;
use std::sync::Arc;
use tokio::signal;

#[tokio::main]
async fn main() -> Result<(), Error> {
    setup_logger();

    info!("Started initializing tracer provider");
    let provider = init_tracer_provider()?;
    opentelemetry::global::set_tracer_provider(provider);
    let tracer = Arc::new(opentelemetry::global::tracer("api"));
    info!("Completed initializing tracer provider");

    info!("Started initializing database connection");
    let url: &str =
        "postgresql://postgres:%40@localhost:15432/mmm";
    init_database_connection(&url, 5)
        .await
        .expect("Failed to initialize database connection");
    let db_pool: &'static DbPool = database::pool();
    info!("Completed initializing database connection");

    info!("Started initializing local cache");
    CacheRegistry::init();    
    let local_caches: Arc<CacheRegistry> = CacheRegistry::global().clone();
    info!("Completed initializing local cache");

    // Initialize services
    let health_svc: Arc<dyn HealthcheckTrait> =
        Arc::new(HealthcheckService::new());
    let auth_svc: Arc<dyn AuthenticationTrait> =
        Arc::new(AuthenticationService::new());

    let state =
        AppState::new(health_svc, auth_svc, db_pool, tracer, local_caches);

    let routers = register_routers(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8880").await?;
    axum::serve(listener, routers)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
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
