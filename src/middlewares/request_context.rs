use crate::middlewares::request_id_mw::request_id_from_headers;
use axum::{body::Body, http::Request};
use futures_util::future::BoxFuture;
use std::{
    convert::Infallible,
    task::{Context, Poll},
};
use tokio::task_local;
use tower::{Layer, Service};

#[derive(Debug, Clone)]
pub struct RequestContext {
    pub request_id: String,
    pub subject: Option<String>,
    pub ip: Option<String>,
    pub user_agent: Option<String>,
}

task_local! {
    static REQ_CTX: RequestContext;
}

pub fn with_ctx<R>(f: impl FnOnce(&RequestContext) -> R) -> Option<R> {
    REQ_CTX.try_with(f).ok()
}

#[derive(Clone, Default)]
pub struct RequestContextLayer;
impl<S> Layer<S> for RequestContextLayer {
    type Service = RequestContextMw<S>;
    fn layer(&self, inner: S) -> Self::Service {
        RequestContextMw { inner }
    }
}
#[derive(Clone)]
pub struct RequestContextMw<S> {
    inner: S,
}

impl<S> Service<Request<Body>> for RequestContextMw<S>
where
    S: Service<Request<Body>, Error = Infallible> + Clone + Send + 'static,
    S::Future: Send + 'static,
{
    type Response = S::Response;
    type Error = Infallible;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(
        &mut self, cx: &mut Context<'_>,
    ) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx).map_err(|_| unreachable!())
    }

    fn call(&mut self, mut req: Request<Body>) -> Self::Future {
        let rid = request_id_from_headers(req.headers_mut());
        let subject = hdr(&req, "x-subject");
        let ip = hdr(&req, "x-forwarded-for").or_else(|| hdr(&req, "x-real-ip"));
        let ua = hdr(&req, "user-agent");

        let ctx = RequestContext {
            request_id: rid,
            subject,
            ip,
            user_agent: ua,
        };
        let mut svc = self.inner.clone();
        Box::pin(REQ_CTX.scope(ctx, async move { svc.call(req).await }))
    }
}

fn hdr(req: &Request<Body>, name: &str) -> Option<String> {
    req.headers()
        .get(name)
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string())
}
