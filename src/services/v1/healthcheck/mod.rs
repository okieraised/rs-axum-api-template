use crate::common::api_response::BaseOutput;
use crate::domains::health::HealthcheckTrait;
use async_trait::async_trait;

pub struct HealthcheckService {}

impl HealthcheckService {}

#[async_trait]
impl HealthcheckTrait for HealthcheckService {
    async fn health(&self) -> BaseOutput {
        BaseOutput {
            code: "OK".to_string(),
            message: "OK".to_string(),
            data: None,
            count: None,
            meta: None,
        }
    }

    async fn live(&self) -> BaseOutput {
        BaseOutput {
            code: "OK".to_string(),
            message: "OK".to_string(),
            data: None,
            count: None,
            meta: None,
        }
    }

    async fn ready(&self) -> BaseOutput {
        BaseOutput {
            code: "OK".to_string(),
            message: "OK".to_string(),
            data: None,
            count: None,
            meta: None,
        }
    }

    async fn started(&self) -> BaseOutput {
        BaseOutput {
            code: "OK".to_string(),
            message: "OK".to_string(),
            data: None,
            count: None,
            meta: None,
        }
    }
}