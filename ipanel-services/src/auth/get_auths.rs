use crate::{Service, ServiceError};
use std::sync::Arc;

pub struct GetAuthsServiceArgs {}

pub trait GetAuthsService:
    Service<Args = GetAuthsServiceArgs, Out = (), Error = GetAuthsServiceError>
{
    fn into_service(self) -> Arc<dyn GetAuthsService>;
}

#[async_trait::async_trait]
impl Service for Arc<dyn GetAuthsService> {
    type Args = GetAuthsServiceArgs;
    type Out = ();
    type Error = GetAuthsServiceError;

    async fn run(&self, args: Self::Args) -> Result<Self::Out, Self::Error> {
        self.as_ref().run(args).await
    }
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
    impl GetAuthsService for GetAuthsByUserAndDomain {
        fn into_service(self) -> Arc<dyn GetAuthsService> {
            Arc::new(self)
        }
    }

    #[async_trait::async_trait]
    impl Service for GetAuthsByUserAndDomain {
        type Args = GetAuthsServiceArgs;
        type Out = ();
        type Error = GetAuthsServiceError;

        async fn run(&self, _args: Self::Args) -> Result<Self::Out, Self::Error> {
            Err(GetAuthsServiceError::Unknow)
        }
    }
}
