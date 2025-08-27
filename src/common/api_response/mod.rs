use crate::constants::http::{
    CONTENT_TYPE_JSON, CONTENT_TYPE_PROBLEM_JSON, HEADER_X_REQUEST_ID,
};
use axum::{
    Json,
    body::Body,
    http::{HeaderMap, HeaderValue, Request, StatusCode, header},
    response::{IntoResponse, Response as AxumResponse},
};
use chrono::{SecondsFormat, Utc};
use serde::{Deserialize, Serialize};
use serde::ser::Serializer;
use serde_json::{Map, Value, json};
use tower_layer::Layer;
use tower_service::Service;
use uuid::Uuid;

fn now_millis() -> i64 {
    Utc::now().timestamp_millis()
}

fn now_rfc3339_nano() -> String {
    Utc::now().to_rfc3339_opts(SecondsFormat::Nanos, true)
}

/// BaseOutput is a generic response struct for service
#[derive(Debug, Clone, Serialize)]
pub struct BaseOutput {
    pub code: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<Map<String, Value>>,
}

/// Pagination is a common meta payload for list endpoints.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pagination {
    pub page: i32,
    #[serde(rename = "per_page")]
    pub per_page: i32,
    pub total: i64,
    #[serde(rename = "total_pages")]
    pub total_pages: i32,
}

/// Serialize `data` as `{}` when it would otherwise be `null`.
fn serialize_data_or_empty<T: Serialize, S: Serializer>(data: &T, ser: S) -> Result<S::Ok, S::Error> {
    let mut v = serde_json::to_value(data).map_err(serde::ser::Error::custom)?;
    if v.is_null() {
        v = Value::Object(Map::new());
    }
    v.serialize(ser)
}

/// Response is a generic API envelope.
/// `server_time` is Unix millis; `server_time_iso` is RFC3339Nano (UTC).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(bound(serialize = "T: Serialize"))]
pub struct Response<T> {
    #[serde(rename = "request_id")]
    pub request_id: String,
    pub code: Option<String>,
    pub message: Option<String>,
    #[serde(rename = "server_time")]
    pub server_time: i64,
    #[serde(rename = "server_time_iso")]
    pub server_time_iso: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(serialize_with = "serialize_data_or_empty")]
    pub data: T,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agg: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<Map<String, Value>>,
}

impl<T: Default> Response<T> {
    /// New response with generated request id and timestamps.
    pub fn new() -> Self {
        Self {
            request_id: Uuid::new_v4().to_string(),
            code: None,
            message: None,
            server_time: now_millis(),
            server_time_iso: now_rfc3339_nano(),
            count: None,
            data: T::default(),
            agg: None,
            meta: None,
        }
    }

    /// New response using a given request id.
    pub fn new_with_request_id(req_id: impl Into<String>) -> Self {
        Self {
            request_id: req_id.into(),
            ..Self::new()
        }
    }

    /// OK constructs a success response with data.
    pub fn ok(data: T) -> Self {
        let mut r = Self::new();
        r.data = data;
        r
    }

    /// Error constructs an error response with a code/message.
    pub fn error(code: impl Into<String>, message: impl Into<String>) -> Self {
        let mut r = Self::new();
        r.code = Some(code.into());
        r.message = Some(message.into());
        r
    }

    /// FromError converts an Option<err> into a response.
    /// If `err` is None, returns OK with `T::default()`.
    pub fn from_error<E>(code: impl Into<String>, err: Option<E>) -> Self
    where
        E: std::error::Error,
    {
        let code = code.into();
        match err {
            None => Self::ok(T::default()),
            Some(e) => Self::error(code, e.to_string()),
        }
    }
}

impl<T> Response<T> {
    pub fn with_code(mut self, c: impl Into<String>) -> Self {
        self.code = Some(c.into());
        self
    }
    pub fn with_message(mut self, m: impl Into<String>) -> Self {
        self.message = Some(m.into());
        self
    }
    pub fn with_count(mut self, n: i32) -> Self {
        if n > 0 {
            self.count = Some(n);
        }
        self
    }

    pub fn with_data(mut self, data: T) -> Self {
        self.data = data;
        self
    }

    pub fn with_agg(mut self, agg: impl Into<Value>) -> Self {
        self.agg = Some(agg.into());
        self
    }
    pub fn with_meta_kv(
        mut self, k: impl Into<String>, v: impl Into<Value>,
    ) -> Self {
        let map = self.meta.get_or_insert_with(Map::new);
        map.insert(k.into(), v.into());
        self
    }
    pub fn with_meta(mut self, m: Map<String, Value>) -> Self {
        let map = self.meta.get_or_insert_with(Map::new);
        for (k, v) in m {
            map.insert(k, v);
        }
        self
    }
    pub fn with_pagination(self, p: Pagination) -> Self {
        self.with_meta_kv(
            "pagination",
            serde_json::to_value(p).unwrap_or(Value::Null),
        )
    }

    pub fn populate(
        mut self, code: impl Into<String>, message: impl Into<String>, data: T,
        meta: Option<Value>, count: Option<i32>,
    ) -> Self {
        self.code = Some(code.into());
        self.message = Some(message.into());
        self.data = data;

        if let Some(m) = meta {
            match m {
                Value::Object(obj) => {
                    self = self.with_meta(obj);
                },
                other => {
                    self = self.with_meta_kv("meta", other);
                },
            }
        }
        if let Some(n) = count {
            if n > 0 {
                self.count = Some(n);
            }
        }
        self
    }

    pub fn with_status(self, status: StatusCode) -> AxumResponse
    where
        T: Serialize,
    {
        build_json_response(status, Some(&self.request_id), &self)
    }
}

impl<T> IntoResponse for Response<T>
where
    T: Serialize,
{
    fn into_response(self) -> AxumResponse {
        build_json_response(StatusCode::OK, Some(&self.request_id), &self)
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct ProblemJson<'a> {
    #[serde(rename = "type")]
    pub typ: &'a str,
    pub title: &'a str,
    pub status: u16,
    pub detail: &'a str,
    pub instance: &'a str,
    #[serde(rename = "server_time")]
    pub server_time: i64,
    #[serde(rename = "server_time_iso")]
    pub server_time_iso: String,
}

/// Build an Axum `problem+json` response (like WriteProblemJSON in Go).
pub fn write_problem_json(
    status: StatusCode, typ: &str, title: &str, detail: &str, instance: &str,
) -> AxumResponse {
    let payload = ProblemJson {
        typ,
        title,
        status: status.as_u16(),
        detail,
        instance,
        server_time: now_millis(),
        server_time_iso: now_rfc3339_nano(),
    };
    let mut headers = HeaderMap::new();
    headers.insert(
        header::CONTENT_TYPE,
        HeaderValue::from_static(CONTENT_TYPE_PROBLEM_JSON),
    );
    (status, headers, Json(payload)).into_response()
}

fn build_json_response<T: Serialize>(
    status: StatusCode, request_id: Option<&str>, payload: &T,
) -> AxumResponse {
    let mut headers = HeaderMap::new();
    headers.insert(
        header::CONTENT_TYPE,
        HeaderValue::from_static(CONTENT_TYPE_JSON),
    );

    if let Some(rid) = request_id {
        if !rid.is_empty() {
            if let Ok(v) = HeaderValue::from_str(rid) {
                headers.insert(HEADER_X_REQUEST_ID, v);
            }
        }
    }

    (status, headers, Json(payload)).into_response()
}
