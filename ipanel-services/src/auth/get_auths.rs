use crate::{Service, ServiceError};
use std::sync::Arc;

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

pub mod impls {
    use super::{GetAuthsService, GetAuthsServiceArgs, GetAuthsServiceError, Service};
    use std::sync::Arc;

    pub struct GetAuthsByUserAndDomain {}
    impl Service for GetAuthsByUserAndDomain {}

    impl Default for GetAuthsByUserAndDomain {
        fn default() -> Self {
            Self::new()
        }
    }

    impl GetAuthsByUserAndDomain {
        pub fn new() -> Self {
            Self {}
        }
    }

    #[async_trait::async_trait]
    impl GetAuthsService for GetAuthsByUserAndDomain {
        fn build(self) -> Arc<dyn GetAuthsService> {
            Arc::new(self)
        }

        async fn run(&self, _args: GetAuthsServiceArgs) -> Result<(), GetAuthsServiceError> {
            Err(GetAuthsServiceError::Unknow)
        }
    }
}
