use crate::domains::authentication::AuthenticationTrait;
use crate::domains::health::HealthcheckTrait;
use once_cell::sync::OnceCell;
use openidconnect::core::CoreClient;
use opentelemetry::global::BoxedTracer;
use std::sync::Arc;

static OIDC: OnceCell<CoreClient> = OnceCell::new();
static HEALTH: OnceCell<Arc<dyn HealthcheckTrait>> = OnceCell::new();
static AUTH: OnceCell<Arc<dyn AuthenticationTrait>> = OnceCell::new();
static TRACER: OnceCell<Arc<BoxedTracer>> = OnceCell::new();

pub fn set_oidc(c: CoreClient) {
    let _ = OIDC.set(c);
}
pub fn set_health(h: Arc<dyn HealthcheckTrait>) {
    let _ = HEALTH.set(h);
}
pub fn set_auth(a: Arc<dyn AuthenticationTrait>) {
    let _ = AUTH.set(a);
}
pub fn set_tracer(t: Arc<BoxedTracer>) {
    let _ = TRACER.set(t);
}

pub fn oidc() -> &'static CoreClient {
    OIDC.get()
        .expect("OIDC client not set; call app_registry::set_oidc(...) first")
}
pub fn health() -> &'static Arc<dyn HealthcheckTrait> {
    HEALTH.get().expect(
        "Health service not set; call app_registry::set_health(...) first",
    )
}
pub fn auth() -> &'static Arc<dyn AuthenticationTrait> {
    AUTH.get()
        .expect("Auth service not set; call app_registry::set_auth(...) first")
}
pub fn tracer() -> Arc<BoxedTracer> {
    TRACER
        .get()
        .expect("Tracer not set; call app_registry::set_tracer(...) first")
        .clone()
}
