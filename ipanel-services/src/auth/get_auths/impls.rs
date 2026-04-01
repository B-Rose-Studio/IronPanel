use super::{GetAuthsService, GetAuthsServiceArgs, GetAuthsServiceError, Service};
use std::sync::Arc;

pub use surreal::*;
mod surreal {
    use super::*;

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
