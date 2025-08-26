mod healthcheck;
mod authentication;

use crate::web::api::app_state::AppState;
use crate::web::api::v1::healthcheck::{HealthcheckDeps, new_healthcheck_router};
use axum::Router;
use crate::web::api::v1::authentication::{new_authentication_router, AuthenticationDeps};

pub fn register_v1_routers(state: AppState) -> Router {
    let healthcheck_state = HealthcheckDeps {
        healthcheck: state.healthcheck.clone(),
    };

    let authentication_state = AuthenticationDeps {
        authentication: state.authentication.clone(),
        oidc: state.oidc.clone(),
    };

    Router::new().
        nest("/health", new_healthcheck_router(healthcheck_state)).
        nest("/auth", new_authentication_router(authentication_state))
}
