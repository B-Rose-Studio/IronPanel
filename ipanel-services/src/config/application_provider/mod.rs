use crate::{Service, ServiceError};
use ipanel_domain::config::application::ApplicationConfig;
use std::sync::Arc;

mod impls;
pub use impls::*;

#[async_trait::async_trait]
pub trait AppConfigProviderService: Service {
    fn build(self) -> Arc<dyn AppConfigProviderService>;
    async fn run(&self) -> Result<ApplicationConfig, AppConfigProviderError>;
}

#[derive(Debug)]
pub enum AppConfigProviderError {
    ParseError(String),
    NotFound(String),
    Unknow,
}

impl ServiceError for AppConfigProviderError {
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
