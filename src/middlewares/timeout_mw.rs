use crate::common::api_response::Response;
use crate::middlewares::request_id_mw::request_id_from_headers;
use axum::{
    body::Body,
    http::{Request, StatusCode},
    response::Response as AxumResponse,
};
use futures_util::future::BoxFuture;
use std::{
    convert::Infallible,
    task::{Context, Poll},
    time::Duration,
};
use tokio::time::timeout;
use tower::{Layer, Service};

#[derive(Clone, Copy, Debug)]
pub struct TimeoutLayer {
    duration: Duration,
}

impl TimeoutLayer {
    pub fn new(duration: Duration) -> Self {
        Self { duration }
    }
}

impl<S> Layer<S> for TimeoutLayer {
    type Service = TimeoutMiddleware<S>;
    fn layer(&self, inner: S) -> Self::Service {
        TimeoutMiddleware {
            inner,
            duration: self.duration,
        }
    }
}

#[derive(Clone)]
pub struct TimeoutMiddleware<S> {
    inner: S,
    duration: Duration,
}

impl<S> Service<Request<Body>> for TimeoutMiddleware<S>
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

    fn call(&mut self, req: Request<Body>) -> Self::Future {
        let mut svc = self.inner.clone();
        let dur = self.duration;

        let mut headers = req.headers().clone();
        let method = req.method().clone();
        let uri = req.uri().clone();
        let req_id = request_id_from_headers(&mut headers);

        Box::pin(async move {
            match timeout(dur, svc.call(req)).await {
                Ok(Ok(res)) => Ok(res),
                Ok(Err(_)) => {
                    let resp =
                        Response::<serde_json::Value>::new_with_request_id(
                            req_id,
                        )
                        .with_code("INTERNAL_ERROR")
                        .with_message("Unhandled internal error")
                        .with_status(StatusCode::INTERNAL_SERVER_ERROR);
                    Ok(resp)
                },
                Err(_) => {
                    let resp =
                        Response::<serde_json::Value>::new_with_request_id(
                            req_id,
                        )
                        .with_code("TIMEOUT")
                        .with_message("Request timed out")
                        .with_meta_kv("timeout_ms", dur.as_millis() as i64)
                        .with_meta_kv("path", uri.path())
                        .with_meta_kv("method", method.to_string())
                        .with_status(StatusCode::GATEWAY_TIMEOUT);
                    Ok(resp)
                },
            }
        })
    }
}
