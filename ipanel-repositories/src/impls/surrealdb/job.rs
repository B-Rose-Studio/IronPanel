use crate::{
    DBClient, ListMethod, Repository, RepositoryError, RepositoryResult,
    interfaces::job::JobRepository, surrealdb::dtos::job::JobRecord,
};
use ipanel_domain::models::job::{Job, JobId};
use surrealdb::{Surreal, engine::remote::ws::Client, types::RecordId};

pub struct SurrealJobRepository {
    db: DBClient<Surreal<Client>>,
}

impl SurrealJobRepository {
    pub fn new(db: DBClient<Surreal<Client>>) -> Self {
        Self { db }
    }
}

#[async_trait::async_trait]
impl Repository for SurrealJobRepository {
    type Entity = Job;
    type Id = JobId;

    async fn list(&self, method: ListMethod) -> RepositoryResult<Vec<Job>> {
        let result = match method {
            ListMethod::All => self.db.select("jobs").await,
            ListMethod::Pagined { page, qtd } => {
                let mut response = self
                    .db
                    .query("SELECT * FROM jobs LIMIT $qtd START $page")
                    .bind(("qtd", qtd))
                    .bind(("page", page))
                    .await
                    .map_err(|_| RepositoryError::DataError)?;

                response.take(0)
            }
        };

        match result {
            Ok(records_list) => {
                let records: Vec<JobRecord> = records_list;
                let list: Vec<Job> = records.iter().map(|record| record.to_entity()).collect();
                Ok(list)
            }
            Err(_) => Err(RepositoryError::DatabaseConnection),
        }
    }

    async fn find_by_id(&self, id: JobId) -> RepositoryResult<Job> {
        let record: Option<JobRecord> = self
            .db
            .select(("jobs", id.0.clone()))
            .await
            .map_err(|_| RepositoryError::DatabaseConnection)?;

        match record {
            Some(r) => Ok(r.to_entity()),
            None => Err(RepositoryError::EntityNotFound(id.0.clone())),
        }
    }

    async fn create(&self, entity: Job) -> RepositoryResult<Job> {
        use surrealdb::types::RecordIdKey;

        let record_data = JobRecord {
            id: RecordId::new("jobs", RecordIdKey::rand()),
            name: entity.name,
            description: entity.description,
            content: entity.content,
            params: entity.params,
        };

        let created: Option<JobRecord> = self
            .db
            .create("jobs")
            .content(record_data)
            .await
            .map_err(|e| {
                println!("{e:?}");
                RepositoryError::DataError
            })?;

        Ok(created.unwrap().to_entity())
    }

    async fn update(&self, entity: Job) -> RepositoryResult<Job> {
        let id = entity.id.clone().ok_or(RepositoryError::DataError)?;

        let record_data = JobRecord {
            id: RecordId::new("jobs", id.0.clone()),
            name: entity.name,
            description: entity.description,
            content: entity.content,
            params: entity.params,
        };

        let record: Option<JobRecord> = self
            .db
            .update(("jobs", id.0.clone()))
            .content(record_data)
            .await
            .map_err(|_| RepositoryError::DataError)?;

        match record {
            Some(r) => Ok(r.to_entity()),
            None => Err(RepositoryError::EntityNotFound(id.0.clone())),
        }
    }

    async fn delete(&self, id: JobId) -> RepositoryResult<Job> {
        let record: Option<JobRecord> = self
            .db
            .delete(("jobs", id.0.clone()))
            .await
            .map_err(|_| RepositoryError::DataError)?;

        match record {
            Some(r) => Ok(r.to_entity()),
            None => Err(RepositoryError::EntityNotFound(id.0.clone())),
        }
    }
}

#[async_trait::async_trait]
impl JobRepository for SurrealJobRepository {}
