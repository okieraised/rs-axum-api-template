use crate::domains::authentication::AuthenticationTrait;
use axum::Router;
use axum::routing::{get, post};
use openidconnect::core::CoreClient;
use std::sync::Arc;

async fn x() {}

#[derive(Clone)]
pub struct AuthenticationDeps {
    pub authentication: Arc<dyn AuthenticationTrait>,
    // pub oidc: CoreClient,
    // pub tracer: Tracer,
    // pub logger: Arc<Logger>,
}

pub fn new_authentication_router(state: AuthenticationDeps) -> Router {
    let oidc_router = Router::new()
        .route("/oidc/callback", get(x))
        .route("/oidc/redirect", get(x))
        .with_state(state.clone());

    let basic_router = Router::new()
        .route("/login", post(x))
        .route("/logout", post(x))
        .route("/refresh", post(x))
        .with_state(state.clone());

    Router::new().merge(oidc_router).merge(basic_router)
}
