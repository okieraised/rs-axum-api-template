use crate::common::api_response::BaseOutput;
use crate::domains::health::HealthcheckTrait;
use async_trait::async_trait;
use crate::common;

pub struct HealthcheckService {}

impl HealthcheckService {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl HealthcheckTrait for HealthcheckService {
    async fn health(&self) -> common::errors::Result<BaseOutput> {
        Ok(BaseOutput {
            code: "OK".to_string(),
            message: "OK".to_string(),
            data: None,
            count: None,
            meta: None,
        })
    }

    async fn live(&self) -> common::errors::Result<BaseOutput> {
        Ok(BaseOutput {
            code: "OK".to_string(),
            message: "OK".to_string(),
            data: None,
            count: None,
            meta: None,
        })
    }

    async fn ready(&self) -> common::errors::Result<BaseOutput> {
        Ok(BaseOutput {
            code: "OK".to_string(),
            message: "OK".to_string(),
            data: None,
            count: None,
            meta: None,
        })
    }

    async fn started(&self) -> common::errors::Result<BaseOutput> {
        Ok(BaseOutput {
            code: "OK".to_string(),
            message: "OK".to_string(),
            data: None,
            count: None,
            meta: None,
        })
    }
}
