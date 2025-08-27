use crate::common::api_response::BaseOutput;
use async_trait::async_trait;
use crate::common;

#[async_trait]
pub trait HealthcheckTrait: Send + Sync {
    async fn health(&self) -> common::errors::Result<BaseOutput>;
    async fn live(&self) -> common::errors::Result<BaseOutput>;
    async fn ready(&self) -> common::errors::Result<BaseOutput>;
    async fn started(&self) -> common::errors::Result<BaseOutput>;
}
