use crate::{Service, ServiceError};
use std::sync::Arc;

mod impls;
pub use impls::*;

pub struct GetAuthsServiceArgs {}

#[async_trait::async_trait]
pub trait GetAuthsService: Service {
    fn build(self) -> Arc<dyn GetAuthsService>;
    async fn run(&self, args: GetAuthsServiceArgs) -> Result<(), GetAuthsServiceError>;
}

#[derive(Debug)]
pub enum GetAuthsServiceError {
    Unknow,
}

impl ServiceError for GetAuthsServiceError {
    fn code(&self) -> String {
        todo!()
    }

    fn content(&self) -> &impl serde::Serialize {
        &()
    }

    fn description(&self) -> String {
        todo!()
    }
}
