use once_cell::sync::Lazy;
use std::{collections::HashMap, path::Path};

pub const CONTENT_TYPE_OCTET_STREAM: &str = "application/octet-stream";
pub const CONTENT_TYPE_JSON: &str = "application/json";
pub const CONTENT_TYPE_PROBLEM_JSON: &str = "application/problem+json";
pub const CONTENT_TYPE_LD_JSON: &str = "application/ld+json";
pub const CONTENT_TYPE_ND_JSON: &str = "application/x-ndjson";
pub const CONTENT_TYPE_FORM: &str = "application/x-www-form-urlencoded";
pub const CONTENT_TYPE_MULTIPART: &str = "multipart/form-data";
pub const CONTENT_TYPE_HTML: &str = "text/html; charset=utf-8";
pub const CONTENT_TYPE_TEXT_UTF8: &str = "text/plain; charset=utf-8";
pub const CONTENT_TYPE_TEXT: &str = "text/plain";
pub const CONTENT_TYPE_XML: &str = "application/xml"; // prefer over text/xml
pub const CONTENT_TYPE_PDF: &str = "application/pdf";
pub const CONTENT_TYPE_ZIP: &str = "application/zip";
pub const CONTENT_TYPE_GZIP: &str = "application/gzip";
pub const CONTENT_TYPE_TAR: &str = "application/x-tar";
pub const CONTENT_TYPE_SEVEN_ZIP: &str = "application/x-7z-compressed";

pub const CONTENT_TYPE_JPEG: &str = "image/jpeg";
pub const CONTENT_TYPE_PNG: &str = "image/png";
pub const CONTENT_TYPE_GIF: &str = "image/gif";
pub const CONTENT_TYPE_WEBP: &str = "image/webp";
pub const CONTENT_TYPE_SVG: &str = "image/svg+xml";
pub const CONTENT_TYPE_AVIF: &str = "image/avif";

pub const CONTENT_TYPE_MP4: &str = "video/mp4";
pub const CONTENT_TYPE_MPEG: &str = "video/mpeg";
pub const CONTENT_TYPE_WEBM: &str = "video/webm";

pub const CONTENT_TYPE_RAR: &str = "application/x-rar-compressed";
pub const CONTENT_TYPE_JS: &str = "application/javascript";
pub const CONTENT_TYPE_CSS: &str = "text/css";
pub const CONTENT_TYPE_WASM: &str = "application/wasm";

pub const HEADER_ACCEPT: &str = "Accept";
pub const HEADER_ACCEPT_ENCODING: &str = "Accept-Encoding";
pub const HEADER_ACCEPT_LANGUAGE: &str = "Accept-Language";
pub const HEADER_AUTHORIZATION: &str = "Authorization";
pub const HEADER_CACHE_CONTROL: &str = "Cache-Control";
pub const HEADER_CONNECTION: &str = "Connection";
pub const HEADER_CONTENT_DISPOSITION: &str = "Content-Disposition";
pub const HEADER_CONTENT_ENCODING: &str = "Content-Encoding";
pub const HEADER_CONTENT_LANGUAGE: &str = "Content-Language";
pub const HEADER_CONTENT_LENGTH: &str = "Content-Length";
pub const HEADER_CONTENT_LOCATION: &str = "Content-Location";
pub const HEADER_CONTENT_MD5: &str = "Content-MD5";
pub const HEADER_CONTENT_TYPE: &str = "Content-Type";
pub const HEADER_CONTENT_DIGEST: &str = "Content-Digest";
pub const HEADER_CONTENT_TRANSFER_ENCODING: &str = "Content-Transfer-Encoding";
pub const HEADER_ETAG: &str = "ETag";
pub const HEADER_EXPIRES: &str = "Expires";
pub const HEADER_HOST: &str = "Host";
pub const HEADER_IF_MATCH: &str = "If-Match";
pub const HEADER_IF_NONE_MATCH: &str = "If-None-Match";
pub const HEADER_IF_MODIFIED_SINCE: &str = "If-Modified-Since";
pub const HEADER_IF_UNMODIFIED_SINCE: &str = "If-Unmodified-Since";
pub const HEADER_LAST_MODIFIED: &str = "Last-Modified";
pub const HEADER_LINK: &str = "Link";
pub const HEADER_LOCATION: &str = "Location";
pub const HEADER_ORIGIN: &str = "Origin";
pub const HEADER_PRAGMA: &str = "Pragma";
pub const HEADER_RANGE: &str = "Range";
pub const HEADER_RETRY_AFTER: &str = "Retry-After";
pub const HEADER_SERVER: &str = "Server";
pub const HEADER_TRAILER: &str = "Trailer";
pub const HEADER_TRANSFER_ENCODING: &str = "Transfer-Encoding";
pub const HEADER_UPGRADE: &str = "Upgrade";
pub const HEADER_USER_AGENT: &str = "User-Agent";
pub const HEADER_VARY: &str = "Vary";
pub const HEADER_VIA: &str = "Via";
pub const HEADER_WWW_AUTHENTICATE: &str = "WWW-Authenticate";

// Cookies / proxy
pub const HEADER_COOKIE: &str = "Cookie";
pub const HEADER_SET_COOKIE: &str = "Set-Cookie";
pub const HEADER_PROXY_AUTH: &str = "Proxy-Authorization";
pub const HEADER_PROXY_AUTHN: &str = "Proxy-Authenticate";

// CORS / proxy-related
pub const HEADER_ACCESS_CONTROL_ALLOW_ORIGIN: &str =
    "Access-Control-Allow-Origin";
pub const HEADER_ACCESS_CONTROL_ALLOW_METHODS: &str =
    "Access-Control-Allow-Methods";
pub const HEADER_ACCESS_CONTROL_ALLOW_HEADERS: &str =
    "Access-Control-Allow-Headers";
pub const HEADER_ACCESS_CONTROL_ALLOW_CREDENTIALS: &str =
    "Access-Control-Allow-Credentials";
pub const HEADER_ACCESS_CONTROL_EXPOSE_HEADERS: &str =
    "Access-Control-Expose-Headers";
pub const HEADER_ACCESS_CONTROL_MAX_AGE: &str = "Access-Control-Max-Age";
pub const HEADER_ACCESS_CONTROL_REQUEST_HEADERS: &str =
    "Access-Control-Request-Headers";
pub const HEADER_ACCESS_CONTROL_REQUEST_METHOD: &str =
    "Access-Control-Request-Method";

pub const HEADER_X_FORWARDED_FOR: &str = "X-Forwarded-For";
pub const HEADER_X_FORWARDED_HOST: &str = "X-Forwarded-Host";
pub const HEADER_X_FORWARDED_PROTO: &str = "X-Forwarded-Proto";
pub const HEADER_X_FORWARDED_SCHEME: &str = "X-Forwarded-Scheme";

// Common X- headers
pub const HEADER_X_API_KEY: &str = "X-API-Key";
pub const HEADER_X_REQUEST_ID: &str = "X-Request-ID";
pub const HEADER_X_REQUESTED_WITH: &str = "X-Requested-With";
pub const HEADER_X_RATE_LIMIT_LIMIT: &str = "X-RateLimit-Limit";
pub const HEADER_X_RATE_LIMIT_REMAINING: &str = "X-RateLimit-Remaining";
pub const HEADER_X_RATE_LIMIT_RESET: &str = "X-RateLimit-Reset";
pub const HEADER_X_RATE_LIMIT_WINDOW: &str = "X-RateLimit-Window";
pub const HEADER_X_RATE_LIMIT_COUNT: &str = "X-RateLimit-Count";
pub const HEADER_X_CONTENT_TYPE_OPTIONS: &str = "X-Content-Type-Options";
pub const HEADER_X_FRAME_OPTIONS: &str = "X-Frame-Options";
pub const HEADER_X_XSS_PROTECTION: &str = "X-XSS-Protection";
pub const HEADER_X_LICENSE_CHECKSUM: &str = "X-License-Checksum";
pub const HEADER_X_MACHINE_CHECKSUM: &str = "X-Machine-Checksum";

static EXTRA_MIME_MAP: Lazy<HashMap<&'static str, &'static str>> =
    Lazy::new(|| {
        HashMap::from([
            ("svg", CONTENT_TYPE_SVG),
            ("json", CONTENT_TYPE_JSON),
            ("jsonl", CONTENT_TYPE_ND_JSON),
            ("ndjson", CONTENT_TYPE_ND_JSON),
            ("wasm", CONTENT_TYPE_WASM),
            ("webp", CONTENT_TYPE_WEBP),
            ("avif", CONTENT_TYPE_AVIF),
            ("mjs", CONTENT_TYPE_JS),
            ("md", "text/markdown; charset=utf-8"),
            ("yml", "application/yaml"),
            ("yaml", "application/yaml"),
        ])
    });

pub fn guess_content_type(path: impl AsRef<Path>) -> &'static str {
    let path = path.as_ref();
    if let Some(ext) = path
        .extension()
        .and_then(|s| s.to_str())
        .map(|s| s.to_ascii_lowercase())
    {
        if let Some(ct) = EXTRA_MIME_MAP.get(ext.as_str()) {
            return ct;
        }
        if let Some(mt) = mime_guess::from_path(path).first_raw() {
            return mt;
        }
    }
    CONTENT_TYPE_OCTET_STREAM
}

/// Lookup by extension (no leading dot). Same precedence as `guess_content_type`.
pub fn content_type_from_ext(ext: &str) -> &'static str {
    let e = ext.trim_start_matches('.').to_ascii_lowercase();
    if let Some(ct) = EXTRA_MIME_MAP.get(e.as_str()) {
        return ct;
    }
    if let Some(mt) = mime_guess::from_ext(&e).first_raw() {
        return mt;
    }
    CONTENT_TYPE_OCTET_STREAM
}
