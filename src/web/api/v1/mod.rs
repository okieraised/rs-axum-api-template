mod authentication;
mod healthcheck;

use crate::web::api::app_state::AppState;
use crate::web::api::v1::authentication::{
    AuthenticationDeps, new_authentication_router,
};
use crate::web::api::v1::healthcheck::{HealthcheckDeps, new_healthcheck_router};
use axum::Router;

pub fn register_v1_routers(state: AppState) -> Router {
    let healthcheck_state = HealthcheckDeps::new(
        state.healthcheck.clone(),
        state.tracer.clone(),
        state.logger,
    );

    let authentication_state = AuthenticationDeps {
        authentication: state.authentication.clone(),
        // oidc: state.oidc.clone(),
    };

    Router::new()
        .nest("/health", new_healthcheck_router(healthcheck_state))
        .nest("/auth", new_authentication_router(authentication_state))
}
