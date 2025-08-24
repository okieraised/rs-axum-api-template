use crate::middlewares::request_id_mw::request_id_from_headers;
use axum::{
    body::Body,
    extract::MatchedPath,
    http::{HeaderMap, Request},
    response::Response as AxumResponse,
};
use futures_util::future::BoxFuture;
use std::{
    convert::Infallible,
    net::SocketAddr,
    task::{Context, Poll},
    time::Instant,
};
use tower::{Layer, Service};
use tracing::{error, info};

#[derive(Clone, Default)]
pub struct RequestLoggingLayer;

impl<S> Layer<S> for RequestLoggingLayer {
    type Service = RequestLoggingMiddleware<S>;
    fn layer(&self, inner: S) -> Self::Service {
        RequestLoggingMiddleware { inner }
    }
}

#[derive(Clone)]
pub struct RequestLoggingMiddleware<S> {
    inner: S,
}

impl<S> Service<Request<Body>> for RequestLoggingMiddleware<S>
where
    S: Service<Request<Body>, Response = AxumResponse, Error = Infallible>
        + Clone
        + Send
        + 'static,
    S::Future: Send + 'static,
{
    type Response = AxumResponse;
    type Error = Infallible;
    type Future = BoxFuture<'static, Result<AxumResponse, Infallible>>;

    fn poll_ready(
        &mut self, cx: &mut Context<'_>,
    ) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx).map_err(|_| unreachable!())
    }

    fn call(&mut self, mut req: Request<Body>) -> Self::Future {
        let mut svc = self.inner.clone();

        let started = Instant::now();
        let method = req.method().clone();
        let uri = req.uri().clone();
        let mut headers = req.headers().clone();

        Box::pin(async move {
            let res = svc.call(req).await?;

            let status = res.status().as_u16();
            let latency_ms = started.elapsed().as_millis() as i64;

            let full_path = res
                .extensions()
                .get::<MatchedPath>()
                .map(|m| m.as_str().to_string())
                .unwrap_or_default();

            let path = uri.path().to_string();
            let query = uri.query().unwrap_or("").to_string();
            let user_agent = header_str(&headers, "user-agent");
            let host = header_str(&headers, "host");

            let request_id = request_id_from_headers(&mut headers);
            let subject = header_str(&headers, "x-subject");

            let client_ip = client_ip_from_headers(&headers)
                .or_else(|| {
                    res.extensions()
                        .get::<axum::extract::ConnectInfo<SocketAddr>>()
                        .map(|ci| ci.0.ip().to_string())
                })
                .unwrap_or_default();

            if (500..=599).contains(&status) {
                error!(
                    request_id = %request_id,
                    subject = %subject,
                    status = status,
                    method = %method,
                    path = %path,
                    full_path = %full_path,
                    query = %query,
                    ip = %client_ip,
                    user_agent = %user_agent,
                    host = %host,
                    latency_ms = latency_ms,
                    message = %path,
                    "",
                );
            } else {
                info!(
                    request_id = %request_id,
                    subject = %subject,
                    status = status,
                    method = %method,
                    path = %path,
                    full_path = %full_path,
                    query = %query,
                    ip = %client_ip,
                    user_agent = %user_agent,
                    host = %host,
                    latency_ms = latency_ms,
                    message = %path,
                    "",
                );
            }
            Ok(res)
        })
    }
}
fn header_str(headers: &HeaderMap, key: &str) -> String {
    headers
        .get(key)
        .and_then(|v| v.to_str().ok())
        .unwrap_or_default()
        .to_string()
}

fn client_ip_from_headers(headers: &HeaderMap) -> Option<String> {
    // X-Forwarded-For may contain a list like "client, proxy1, proxy2"
    if let Some(v) = headers.get("x-forwarded-for").and_then(|v| v.to_str().ok())
    {
        if let Some(first) = v.split(',').next() {
            let ip = first.trim();
            if !ip.is_empty() {
                return Some(ip.to_string());
            }
        }
    }
    if let Some(v) = headers.get("x-real-ip").and_then(|v| v.to_str().ok()) {
        let ip = v.trim();
        if !ip.is_empty() {
            return Some(ip.to_string());
        }
    }
    None
}
