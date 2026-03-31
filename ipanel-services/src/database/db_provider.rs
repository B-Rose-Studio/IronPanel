use crate::{Service, ServiceError};
use ipanel_repositories::DBClient;
use std::sync::Arc;

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

pub mod impls {
    use surrealdb::{
        Surreal,
        engine::remote::ws::{Client, Ws},
        opt::auth::Root,
    };

    use super::*;
    use std::sync::Arc;

    pub struct SurrealDatabaseProvider {
        url: String,
        db: String,
        ns: String,
        username: String,
        password: String,
    }
    impl Service for SurrealDatabaseProvider {}

    impl SurrealDatabaseProvider {
        pub fn new(
            url: impl Into<String>,
            db: impl Into<String>,
            ns: impl Into<String>,
            username: impl Into<String>,
            password: impl Into<String>,
        ) -> Self {
            Self {
                url: url.into(),
                db: db.into(),
                ns: ns.into(),
                username: username.into(),
                password: password.into(),
            }
        }
    }

    #[async_trait::async_trait]
    impl DatabaseProviderService<Surreal<Client>> for SurrealDatabaseProvider {
        fn build(self) -> Arc<dyn DatabaseProviderService<Surreal<Client>>> {
            Arc::new(self)
        }

        async fn run(&self) -> Result<DBClient<Surreal<Client>>, DatabaseProviderError> {
            let db = Surreal::new::<Ws>(&self.url)
                .await
                .map_err(|e| DatabaseProviderError::ConnectionError(e.to_string()))?;

            db.signin(Root {
                username: self.username.clone(),
                password: self.password.clone(),
            })
            .await
            .map_err(|e| DatabaseProviderError::AuthenticationError(e.to_string()))?;

            db.use_ns(&self.ns)
                .use_db(&self.db)
                .await
                .map_err(|e| DatabaseProviderError::DatabaseError(e.to_string()))?;

            Ok(DBClient::new(db))
        }
    }
}
