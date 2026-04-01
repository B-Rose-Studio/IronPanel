use super::{DatabaseProviderError, DatabaseProviderService};
use crate::Service;
use ipanel_repositories::DBClient;
use std::sync::Arc;

pub use surreal::*;
mod surreal {
    use super::*;
    use surrealdb::{Surreal, engine::any::Any, opt::auth::Database};

    pub enum DBConfig {
        Params {
            url: String,
            db: String,
            ns: String,
            username: String,
            password: String,
        },

        Env,
    }

    pub struct SurrealDatabaseProvider {
        url: String,
        db: String,
        ns: String,
        username: String,
        password: String,
    }

    impl Service for SurrealDatabaseProvider {}

    impl SurrealDatabaseProvider {
        pub fn new(config: DBConfig) -> Self {
            let (url, db, ns, username, password) = match config {
                DBConfig::Params {
                    url,
                    db,
                    ns,
                    username,
                    password,
                } => (url, db, ns, username, password),

                DBConfig::Env => {
                    let env = "IPANEL_SURREAL";

                    let url = std::env::var(format!("{}_URL", env))
                        .expect("Expect URL connection [IPANEL_SURREAL_URL]");

                    let db = std::env::var(format!("{}_DB", env))
                        .expect("Expect DB name [IPANEL_SURREAL_DB]");

                    let ns = std::env::var(format!("{}_NS", env))
                        .expect("Expect NS name [IPANEL_SURREAL_NS]");

                    let username = std::env::var(format!("{}_USERNAME", env))
                        .expect("Expect username [IPANEL_SURREAL_USERNAME]");

                    let password = std::env::var(format!("{}_PASSWORD", env))
                        .expect("Expect password [IPANEL_SURREAL_PASSWORD]");

                    (url, db, ns, username, password)
                }
            };

            Self {
                url,
                db,
                ns,
                username,
                password,
            }
        }
    }

    #[async_trait::async_trait]
    impl DatabaseProviderService<Surreal<Any>> for SurrealDatabaseProvider {
        fn build(self) -> Arc<dyn DatabaseProviderService<Surreal<Any>>> {
            Arc::new(self)
        }

        async fn run(&self) -> Result<DBClient<Surreal<Any>>, DatabaseProviderError> {
            let db: Surreal<Any> = Surreal::init();

            db.connect(&self.url)
                .await
                .map_err(|e| DatabaseProviderError::ConnectionError(e.to_string()))?;

            db.signin(Database {
                namespace: self.ns.clone(),
                database: self.db.clone(),
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
