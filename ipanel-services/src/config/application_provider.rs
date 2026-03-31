use crate::{Service, ServiceError};
use ipanel_domain::config::application::ApplicationConfig;
use std::sync::Arc;

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

pub mod impls {
    use super::*;
    use ipanel_domain::{config::application::ConfigId, models::group::GroupId};
    use ipanel_repositories::DBClient;
    use surrealdb::{
        Surreal,
        engine::remote::ws::Client,
        types::{RecordId, SurrealValue},
    };

    pub struct SurrealAppConfigProvider {
        db: DBClient<Surreal<Client>>,
    }

    impl Service for SurrealAppConfigProvider {}

    impl SurrealAppConfigProvider {
        pub fn new(db: DBClient<Surreal<Client>>) -> Self {
            Self { db }
        }
    }

    #[derive(SurrealValue, Debug)]
    pub struct AppConfigRecord {
        pub id: RecordId,
        pub auto_registre: bool,
        pub default_group: RecordId,
    }

    impl AppConfigRecord {
        pub fn to_entity(&self) -> ApplicationConfig {
            ApplicationConfig {
                id: Some(ConfigId(
                    self.id.clone().key.into_value().into_string().unwrap(),
                )),
                auto_registre: self.auto_registre,
                default_group: GroupId(
                    self.default_group
                        .clone()
                        .key
                        .into_value()
                        .into_string()
                        .unwrap(),
                ),
            }
        }
    }

    #[async_trait::async_trait]
    impl AppConfigProviderService for SurrealAppConfigProvider {
        fn build(self) -> Arc<dyn AppConfigProviderService> {
            Arc::new(self)
        }

        async fn run(&self) -> Result<ApplicationConfig, AppConfigProviderError> {
            let config: Option<AppConfigRecord> = self
                .db
                .select(("config", "settings"))
                .await
                .map_err(|e| AppConfigProviderError::ParseError(e.to_string()))?;

            let config = config.ok_or(AppConfigProviderError::NotFound(
                "config not record in database".to_string(),
            ))?;

            Ok(config.to_entity())
        }
    }
}
