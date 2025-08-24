use crate::constants::http::HEADER_X_REQUEST_ID;
use axum::{
    body::Body,
    extract::FromRequestParts,
    http::{HeaderMap, HeaderValue, Request, StatusCode, request::Parts},
};
use futures_util::future::ready;
use std::{
    convert::Infallible,
    task::{Context, Poll},
};
use tower::{Layer, Service};
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct RequestId(pub String);

#[derive(Clone, Default)]
pub struct RequestIdLayer;

impl<S> Layer<S> for RequestIdLayer {
    type Service = RequestIdMiddleware<S>;
    fn layer(&self, inner: S) -> Self::Service {
        RequestIdMiddleware { inner }
    }
}

#[derive(Clone)]
pub struct RequestIdMiddleware<S> {
    inner: S,
}

impl<S> Service<Request<Body>> for RequestIdMiddleware<S>
where
    S: Service<Request<Body>, Error = Infallible> + Clone + Send + 'static,
    S::Future: Send + 'static,
{
    type Response = S::Response;
    type Error = Infallible;
    type Future = futures_util::future::BoxFuture<
        'static,
        Result<Self::Response, Self::Error>,
    >;

    fn poll_ready(
        &mut self, cx: &mut Context<'_>,
    ) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx).map_err(|_| unreachable!())
    }

    fn call(&mut self, mut req: Request<Body>) -> Self::Future {
        let rid = request_id_from_headers(req.headers_mut());
        req.extensions_mut().insert(RequestId(rid));
        let mut svc = self.inner.clone();
        Box::pin(async move { svc.call(req).await })
    }
}

pub fn request_id_from_headers(headers: &mut HeaderMap) -> String {
    if let Some(v) = headers
        .get(HEADER_X_REQUEST_ID)
        .and_then(|v| v.to_str().ok())
    {
        return v.to_string();
    }
    let rid = Uuid::new_v4().to_string();
    if let Ok(hv) = HeaderValue::from_str(&rid) {
        headers.insert(HEADER_X_REQUEST_ID, hv);
    }
    rid
}

impl<S> FromRequestParts<S> for RequestId
where
    S: Send + Sync,
{
    type Rejection = StatusCode;

    fn from_request_parts(
        parts: &mut Parts, _: &S,
    ) -> impl Future<
        Output = Result<Self, <Self as FromRequestParts<S>>::Rejection>,
    > + Send {
        if let Some(r) = parts.extensions.get::<RequestId>() {
            return ready(Ok(r.clone()));
        }
        if let Some(v) = parts
            .headers
            .get(HEADER_X_REQUEST_ID)
            .and_then(|v| v.to_str().ok())
        {
            return ready(Ok(RequestId(v.to_string())));
        }
        ready(Ok(RequestId(Uuid::new_v4().to_string())))
    }
}
