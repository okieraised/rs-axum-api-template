use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::{
    Resource,
    trace::{SdkTracerProvider, TraceError},
};

use crate::config::env_settings::SERVICE_CONFIGURATION;

pub fn init_tracer_provider() -> Result<SdkTracerProvider, TraceError> {
    let exporter = opentelemetry_otlp::SpanExporter::builder()
        .with_tonic()
        .with_endpoint(SERVICE_CONFIGURATION.otel.uri.clone())
        .build()
        .map_err(|e| TraceError::Other(Box::new(e)))?;

    let resource = Resource::builder_empty()
        .with_service_name(SERVICE_CONFIGURATION.server.name.clone())
        .build();

    let provider = SdkTracerProvider::builder()
        .with_batch_exporter(exporter)
        .with_resource(resource)
        .build();

    Ok(provider)
}
