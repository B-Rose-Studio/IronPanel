use crate::{Service, ServiceError};
use ipanel_repositories::DBClient;
use std::sync::Arc;

mod impls;
pub use impls::*;

#[async_trait::async_trait]
pub trait DatabaseProviderService<T: 'static>: Service {
    fn build(self) -> Arc<dyn DatabaseProviderService<T>>;
    async fn run(&self) -> Result<DBClient<T>, DatabaseProviderError>;
}

#[derive(Debug)]
pub enum DatabaseProviderError {
    AuthenticationError(String),
    ConnectionError(String),
    DatabaseError(String),
}

impl ServiceError for DatabaseProviderError {
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
