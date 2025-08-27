use log::Log;
use openidconnect::core::CoreClient;
use opentelemetry::global::BoxedTracer;
use std::sync::Arc;

use crate::database::{DbPool, pool as db_pool};
use crate::domains::authentication::AuthenticationTrait;
use crate::domains::health::HealthcheckTrait;
use crate::infrastructures::cache::local_cache::CacheRegistry;
use crate::web::api::app_registry;

#[derive(Debug, Clone, Default)]
pub struct RequestLogCtx {
    pub request_id: String,
    pub subject: Option<String>,
}

#[derive(Clone)]
pub struct AppState {
    pub healthcheck: Arc<dyn HealthcheckTrait>,
    pub authentication: Arc<dyn AuthenticationTrait>,

    // pub oidc: CoreClient,
    pub db: &'static DbPool,
    pub tracer: Arc<BoxedTracer>,
    pub logger: &'static dyn Log,
    pub caches: Arc<CacheRegistry>,
}

impl AppState {
    pub fn new(
        healthcheck: Arc<dyn HealthcheckTrait>,
        authentication: Arc<dyn AuthenticationTrait>,
        // oidc: CoreClient,
        db: &'static DbPool,
        tracer: Arc<BoxedTracer>,
        caches: Arc<CacheRegistry>,
    ) -> Self {
        Self {
            healthcheck,
            authentication,
            // oidc,
            db,
            tracer,
            logger: log::logger(),
            caches,
        }
    }

    /// Panic-on-miss constructor: pulls everything from global singletons.
    /// Panics if any dependency was not registered/initialized.
    pub fn from_globals() -> Self {
        Self {
            // oidc: app_registry::oidc().clone(),
            healthcheck: app_registry::health().clone(),
            authentication: app_registry::auth().clone(),
            db: db_pool(),
            tracer: app_registry::tracer(),
            logger: log::logger(),
            caches: CacheRegistry::global().clone(),
        }
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::from_globals()
    }
}
