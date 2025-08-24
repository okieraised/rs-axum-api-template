use crate::common::api_response::Response;
use crate::middlewares::request_id_mw::request_id_from_headers;
use axum::{
    http::{HeaderMap, Method, StatusCode, Uri},
    response::IntoResponse,
};

pub async fn not_found_middleware(
    mut headers: HeaderMap, uri: Uri, method: Method,
) -> impl IntoResponse {
    let req_id = request_id_from_headers(&mut headers);

    Response::<serde_json::Value>::new_with_request_id(req_id)
        .with_code("NOT_FOUND")
        .with_message("No route matches the requested path")
        .with_meta_kv("path", uri.path())
        .with_meta_kv("method", method.to_string())
        .with_meta_kv(
            "host",
            headers
                .get("Host")
                .and_then(|v| v.to_str().ok())
                .unwrap_or(""),
        )
        .with_status(StatusCode::NOT_FOUND)
}
