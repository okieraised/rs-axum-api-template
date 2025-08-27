use async_trait::async_trait;

#[async_trait]
pub trait AuthenticationTrait: Send + Sync {}
