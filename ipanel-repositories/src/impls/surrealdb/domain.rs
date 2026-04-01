use crate::{
    DBClient, ListMethod, Repository, RepositoryError, RepositoryResult,
    interfaces::domain::DomainRepository, surrealdb::dtos::domain::DomainRecord,
};
use ipanel_domain::models::domain::{Domain, DomainId};
use surrealdb::{Surreal, engine::any::Any, types::RecordId};

pub struct SurrealDomainRepository {
    db: DBClient<Surreal<Any>>,
}

impl SurrealDomainRepository {
    pub fn new(db: DBClient<Surreal<Any>>) -> Self {
        Self { db }
    }
}

#[async_trait::async_trait]
impl Repository for SurrealDomainRepository {
    type Entity = Domain;
    type Id = DomainId;

    async fn list(&self, method: ListMethod) -> RepositoryResult<Vec<Domain>> {
        let result = match method {
            ListMethod::All => self.db.select("domains").await,
            ListMethod::Pagined { page, qtd } => {
                let mut response = self
                    .db
                    .query("SELECT * FROM domains LIMIT $qtd START $page")
                    .bind(("qtd", qtd))
                    .bind(("page", page))
                    .await
                    .map_err(|_| RepositoryError::DataError)?;

                response.take(0)
            }
        };

        match result {
            Ok(records_list) => {
                let records: Vec<DomainRecord> = records_list;
                let list: Vec<Domain> = records.iter().map(|record| record.to_entity()).collect();
                Ok(list)
            }
            Err(_) => Err(RepositoryError::DatabaseConnection),
        }
    }

    async fn find_by_id(&self, id: DomainId) -> RepositoryResult<Domain> {
        let record: Option<DomainRecord> = self
            .db
            .select(("domains", id.0.clone()))
            .await
            .map_err(|_| RepositoryError::DatabaseConnection)?;

        match record {
            Some(r) => Ok(r.to_entity()),
            None => Err(RepositoryError::EntityNotFound(id.0.clone())),
        }
    }

    async fn create(&self, entity: Domain) -> RepositoryResult<Domain> {
        use surrealdb::types::RecordIdKey;

        let record_data = DomainRecord {
            id: RecordId::new("domains", RecordIdKey::rand()),
            name: entity.name,
            active: entity.active,
            url: entity.url,
            user_service: entity.user_service,
            password_user: entity.password_user,
            base_dn: entity.base_dn,
            user_filter: entity.user_filter,
        };

        let created: Option<DomainRecord> = self
            .db
            .create("domains")
            .content(record_data)
            .await
            .map_err(|e| {
                println!("{e:?}");
                RepositoryError::DataError
            })?;

        Ok(created.unwrap().to_entity())
    }

    async fn update(&self, entity: Domain) -> RepositoryResult<Domain> {
        let id = entity.id.clone().ok_or(RepositoryError::DataError)?;

        let record_data = DomainRecord {
            id: RecordId::new("domains", id.0.clone()),
            name: entity.name,
            active: entity.active,
            url: entity.url,
            user_service: entity.user_service,
            password_user: entity.password_user,
            base_dn: entity.base_dn,
            user_filter: entity.user_filter,
        };

        let record: Option<DomainRecord> = self
            .db
            .update(("domains", id.0.clone()))
            .content(record_data)
            .await
            .map_err(|_| RepositoryError::DataError)?;

        match record {
            Some(r) => Ok(r.to_entity()),
            None => Err(RepositoryError::EntityNotFound(id.0.clone())),
        }
    }

    async fn delete(&self, id: DomainId) -> RepositoryResult<Domain> {
        let record: Option<DomainRecord> = self
            .db
            .delete(("domains", id.0.clone()))
            .await
            .map_err(|_| RepositoryError::DataError)?;

        match record {
            Some(r) => Ok(r.to_entity()),
            None => Err(RepositoryError::EntityNotFound(id.0.clone())),
        }
    }
}

#[async_trait::async_trait]
impl DomainRepository for SurrealDomainRepository {}
