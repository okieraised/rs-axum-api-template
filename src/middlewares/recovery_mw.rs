use crate::common::api_response::Response;
use crate::middlewares::request_id_mw::request_id_from_headers;
use axum::{
    body::Body,
    http::{Request, StatusCode},
    response::Response as AxumResponse,
};
use futures_util::FutureExt;
use std::{
    convert::Infallible,
    panic::AssertUnwindSafe,
    task::{Context, Poll},
};
use tower::{Layer, Service};
use tracing::error;

#[derive(Clone, Default)]
pub struct RecoveryLayer;

impl<S> Layer<S> for RecoveryLayer {
    type Service = RecoveryMiddleware<S>;
    fn layer(&self, inner: S) -> Self::Service {
        RecoveryMiddleware { inner }
    }
}

#[derive(Clone)]
pub struct RecoveryMiddleware<S> {
    inner: S,
}

impl<S> Service<Request<Body>> for RecoveryMiddleware<S>
where
    S: Service<Request<Body>, Response = AxumResponse, Error = Infallible>
        + Clone
        + Send
        + 'static,
    S::Future: Send + 'static,
{
    type Response = AxumResponse;
    type Error = Infallible;
    type Future = std::pin::Pin<
        Box<
            dyn futures_util::Future<Output = Result<AxumResponse, Infallible>>
                + Send,
        >,
    >;

    fn poll_ready(
        &mut self, cx: &mut Context<'_>,
    ) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx).map_err(|_| unreachable!())
    }

    fn call(&mut self, req: Request<Body>) -> Self::Future {
        let mut svc = self.inner.clone();

        // Snapshot a few bits for logging/response
        let mut headers = req.headers().clone();
        let method = req.method().clone();
        let uri = req.uri().clone();
        let req_id = request_id_from_headers(&mut headers);

        Box::pin(async move {
            match AssertUnwindSafe(svc.call(req)).catch_unwind().await {
                Ok(Ok(res)) => Ok(res),
                Ok(Err(_)) => {
                    let resp =
                        Response::<serde_json::Value>::new_with_request_id(
                            req_id,
                        )
                        .with_code("INTERNAL_ERROR")
                        .with_message("internal web error")
                        .with_status(StatusCode::INTERNAL_SERVER_ERROR);
                    Ok(resp)
                },
                Err(panic_payload) => {
                    let panic_msg = if let Some(s) =
                        panic_payload.downcast_ref::<&str>()
                    {
                        (*s).to_string()
                    } else if let Some(s) = panic_payload.downcast_ref::<String>()
                    {
                        s.clone()
                    } else {
                        "panic occurred".to_string()
                    };

                    let backtrace = std::backtrace::Backtrace::force_capture();

                    error!(
                        request_id = %req_id,
                        method = %method,
                        path = %uri,
                        panic = %panic_msg,
                        backtrace = %backtrace,
                        "request panicked"
                    );

                    let mut resp =
                        Response::<serde_json::Value>::new_with_request_id(
                            req_id,
                        )
                        .with_code("INTERNAL_ERROR")
                        .with_message("internal web error")
                        .with_meta_kv("path", uri.path())
                        .with_meta_kv("method", method.to_string());

                    if cfg!(debug_assertions) {
                        resp = resp.with_meta_kv("panic", panic_msg);
                    }

                    Ok(resp.with_status(StatusCode::INTERNAL_SERVER_ERROR))
                },
            }
        })
    }
}
