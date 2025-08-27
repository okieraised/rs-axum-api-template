use crate::common::api_response::BaseOutput;
use crate::domains::authentication::AuthenticationTrait;
use async_trait::async_trait;

pub struct AuthenticationService {}

impl AuthenticationService {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl AuthenticationTrait for AuthenticationService {}
