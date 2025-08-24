use crate::common::api_response::BaseOutput;
use async_trait::async_trait;

#[async_trait]
pub trait HealthcheckTrait: Send + Sync {
    async fn health(&self) -> BaseOutput;
    async fn live(&self) -> BaseOutput;
    async fn ready(&self) -> BaseOutput;
    async fn started(&self) -> BaseOutput;
}
