use crate::domains::health::HealthcheckTrait;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub healthcheck: Arc<dyn HealthcheckTrait>,
}
