use crate::domains::health::HealthcheckTrait;
use std::sync::Arc;
use openidconnect::core::CoreClient;
use crate::domains::authentication::AuthenticationTrait;

#[derive(Clone)]
pub struct AppState {
    pub oidc: CoreClient,
    pub healthcheck: Arc<dyn HealthcheckTrait>,
    pub authentication: Arc<dyn AuthenticationTrait>,
}
