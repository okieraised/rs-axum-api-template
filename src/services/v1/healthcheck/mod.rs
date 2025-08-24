use crate::common::api_response::BaseOutput;
use crate::domains::health::HealthcheckTrait;
use async_trait::async_trait;

pub struct HealthcheckService {}

impl HealthcheckService {}

#[async_trait]
impl HealthcheckTrait for HealthcheckService {
    async fn health(&self) -> BaseOutput {
        todo!()
    }

    async fn live(&self) -> BaseOutput {
        todo!()
    }

    async fn ready(&self) -> BaseOutput {
        todo!()
    }

    async fn started(&self) -> BaseOutput {
        todo!()
    }
}