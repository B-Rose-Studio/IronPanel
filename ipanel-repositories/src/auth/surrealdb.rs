use crate::{
    ListMethod, Repository, RepositoryError, RepositoryResult, auth::surrealdb::dtos::AuthRecord,
};
use ipanel_domain::models::auth::{Auth, AuthId};
use std::sync::Arc;
use surrealdb::{Surreal, engine::remote::ws::Client};

mod dtos {
    use ipanel_domain::models::auth::{Auth, AuthId};
    use std::collections::BTreeMap;
    use surrealdb::types::{RecordId, SurrealValue};

    #[derive(SurrealValue)]
    pub struct AuthRecord {
        pub id: RecordId,
        pub name: String,
        pub params: BTreeMap<String, String>,
    }

    impl AuthRecord {
        pub fn into(&self) -> Auth {
            Auth {
                id: Some(AuthId(
                    self.id.clone().key.into_value().into_string().unwrap(),
                )),
                name: self.name.clone(),
                params: self.params.clone(),
            }
        }
    }
}

pub struct SurrealAuthRepository {
    pub db: Arc<Surreal<Client>>,
}

#[async_trait::async_trait]
impl Repository for SurrealAuthRepository {
    type Entity = Auth;
    type Id = AuthId;

    async fn list(&self, method: ListMethod) -> RepositoryResult<Vec<Auth>> {
        let result = match method {
            ListMethod::All => self.db.query("SELECT * FROM auths").await,

            ListMethod::Pagined { page, qtd } => {
                self.db
                    .query(format!("SELECT * FROM auths LIMIT {qtd} START {page}"))
                    .await
            }
        };

        match result {
            Ok(mut results) => {
                let records_list: Vec<AuthRecord> = results.take(0).unwrap_or_default();
                let list: Vec<Auth> = records_list.iter().map(|record| record.into()).collect();
                Ok(list)
            }

            Err(_) => Err(RepositoryError::DataError),
        }
    }

    async fn find_by_id(&self, _id: AuthId) -> RepositoryResult<Auth> {
        todo!()
    }

    async fn create(&self, _entity: Auth) -> RepositoryResult<Auth> {
        todo!()
    }

    async fn update(&self, _entity: Auth) -> RepositoryResult<Auth> {
        todo!()
    }

    async fn delete(&self, _id: AuthId) -> RepositoryResult<Auth> {
        todo!()
    }
}
