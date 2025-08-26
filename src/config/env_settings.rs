use once_cell::sync::Lazy;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
#[serde(default)]
pub struct HTTPConfig {
    /// Service name
    pub name: String,
    /// Service's HTTP port
    pub http_port: u16,
    /// Service's HTTP request timeout duration in seconds
    pub request_timeout_duration: u64,
    /// Service's log level
    pub log_level: String,
}

impl Default for HTTPConfig {
    fn default() -> Self {
        Self {
            name: "example-service".to_string(),
            http_port: 8080,
            request_timeout_duration: 10,
            log_level: "info".to_string(),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(default)]
pub struct OtelConfig {
    /// Otel URI
    pub(crate) uri: String,
}

impl Default for OtelConfig {
    fn default() -> Self {
        Self {
            uri: "localhost:4317".to_string(),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(default)]
pub struct OIDCConfig {
    pub enabled: bool,
    pub client_id: String,
    pub client_secret: String,
    pub scopes: String,
    pub redirect_url: String,
    pub realm: String,
}

impl Default for OIDCConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            client_id: "".to_string(),
            client_secret: "".to_string(),
            scopes: "".to_string(),
            redirect_url: "".to_string(),
            realm: "".to_string(),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct Settings {
    #[serde(default)]
    pub server: HTTPConfig,
    #[serde(default)]
    pub otel: OtelConfig,
}

impl Settings {
    pub fn new() -> Result<Self, config::ConfigError> {
        let _ = dotenvy::dotenv();

        let profile = detect_profile();

        let builder = config::Config::builder()
            .add_source(
                config::File::with_name(".env")
                    .format(config::FileFormat::Ini)
                    .required(false),
            )
            .add_source(
                config::File::with_name("conf/config.toml")
                    .format(config::FileFormat::Toml)
                    .required(false),
            )
            .add_source(
                config::File::with_name(&format!(".{profile}.env"))
                    .format(config::FileFormat::Ini)
                    .required(false),
            )
            .add_source(
                config::File::with_name(&format!("conf/{profile}.config.toml"))
                    .format(config::FileFormat::Toml)
                    .required(false),
            )
            // Use double underscore to nest (e.g., SERVER__HTTP_PORT=8080)
            .add_source(config::Environment::default().separator("__"));

        builder.build()?.try_deserialize()
    }
}

pub static SERVICE_CONFIGURATION: Lazy<Settings> =
    Lazy::new(|| Settings::new().expect("Failed to setup service configuration"));

/// Detects the active configuration profile.
/// Precedence: APP_ENV > RUST_ENV > ENV > (debug=dev, release=prod)
fn detect_profile() -> String {
    let from = |k: &str| std::env::var(k).ok().map(|v| v.to_ascii_lowercase());

    from("APP_ENV")
        .or_else(|| from("RUST_ENV"))
        .or_else(|| from("ENV"))
        .unwrap_or_else(|| {
            if cfg!(debug_assertions) {
                "dev".to_string()
            } else {
                "prod".to_string()
            }
        })
}
