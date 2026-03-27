use crate::{
    ListMethod, Repository, RepositoryError, RepositoryResult, auth::surrealdb::dtos::AuthRecord,
};
use ipanel_domain::models::auth::{Auth, AuthId};
use std::sync::Arc;
use surrealdb::{
    Surreal,
    engine::remote::ws::Client,
    types::{RecordId, RecordIdKey},
};

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
        pub fn to_entity(&self) -> Auth {
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
            ListMethod::All => self.db.select("auths").await,
            ListMethod::Pagined { page, qtd } => {
                let mut response = self
                    .db
                    .query("SELECT * FROM auths LIMIT $qtd START $page")
                    .bind(("qtd", qtd))
                    .bind(("page", page))
                    .await
                    .map_err(|_| RepositoryError::DataError)?;

                response.take(0)
            }
        };

        match result {
            Ok(records_list) => {
                let records: Vec<AuthRecord> = records_list;
                let list: Vec<Auth> = records.iter().map(|record| record.to_entity()).collect();
                Ok(list)
            }
            Err(_) => Err(RepositoryError::DatabaseConnection),
        }
    }

    async fn find_by_id(&self, id: AuthId) -> RepositoryResult<Auth> {
        let record: Option<AuthRecord> = self
            .db
            .select(("auths", id.0.clone()))
            .await
            .map_err(|_| RepositoryError::DatabaseConnection)?;

        match record {
            Some(r) => Ok(r.to_entity()),
            None => Err(RepositoryError::EntityNotFound(id.0.clone())),
        }
    }

    async fn create(&self, entity: Auth) -> RepositoryResult<Auth> {
        let record_data = AuthRecord {
            id: RecordId::new("auths", RecordIdKey::rand()),
            name: entity.name,
            params: entity.params,
        };

        let created: Option<AuthRecord> = self
            .db
            .create("auths")
            .content(record_data)
            .await
            .map_err(|e| {
                println!("{e:?}");
                RepositoryError::DataError
            })?;

        Ok(created.unwrap().to_entity())
    }

    async fn update(&self, entity: Auth) -> RepositoryResult<Auth> {
        let id = entity.id.clone().ok_or(RepositoryError::DataError)?;

        let record_data = AuthRecord {
            id: RecordId::new("auths", id.0.clone()),
            name: entity.name,
            params: entity.params,
        };

        let record: Option<AuthRecord> = self
            .db
            .update(("auths", id.0.clone()))
            .content(record_data)
            .await
            .map_err(|_| RepositoryError::DataError)?;

        match record {
            Some(r) => Ok(r.to_entity()),
            None => Err(RepositoryError::EntityNotFound(id.0.clone())),
        }
    }

    async fn delete(&self, id: AuthId) -> RepositoryResult<Auth> {
        let record: Option<AuthRecord> = self
            .db
            .delete(("auths", id.0.clone()))
            .await
            .map_err(|_| RepositoryError::DataError)?;

        match record {
            Some(r) => Ok(r.to_entity()),
            None => Err(RepositoryError::EntityNotFound(id.0.clone())),
        }
    }
}
