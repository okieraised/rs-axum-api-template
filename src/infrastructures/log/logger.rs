use crate::config::env_settings::SERVICE_CONFIGURATION;
use serde::Serialize;

#[derive(Serialize)]
pub struct LoggerExtraFields {
    pub request_id: String,
}

pub fn setup_logger() {
    let mut log_level: log::LevelFilter = log::LevelFilter::Debug;

    let setting_level = &SERVICE_CONFIGURATION.server.log_level.clone();

    match setting_level.as_str() {
        "trace" => {
            log_level = log::LevelFilter::Trace;
        },
        "debug" => {
            log_level = log::LevelFilter::Debug;
        },
        "warn" => {
            log_level = log::LevelFilter::Warn;
        },
        "error" => {
            log_level = log::LevelFilter::Error;
        },
        _ => {
            log_level = log::LevelFilter::Info;
        },
    };

    env_logger::builder()
        .filter_level(log_level)
        .format_timestamp_micros()
        .format(ecs_logger::format)
        .format_module_path(false)
        .target(env_logger::Target::Stdout)
        .target(env_logger::Target::Stderr)
        .init();
}

#[cfg(test)]
mod tests {
    use super::*;
    use log::info;

    #[test]
    fn test_logger() {
        setup_logger();
        info!("test log info");
    }
}
