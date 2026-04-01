use crate::{
    DBClient, ListMethod, Repository, RepositoryError, RepositoryResult,
    interfaces::log::LogRepository,
    surrealdb::dtos::log::{LogRecord, LogSourceRecord},
};
use ipanel_domain::models::log::{Log, LogId, LogSource};
use surrealdb::{Surreal, engine::any::Any, types::RecordId};

pub struct SurrealLogRepository {
    db: DBClient<Surreal<Any>>,
}

impl SurrealLogRepository {
    pub fn new(db: DBClient<Surreal<Any>>) -> Self {
        Self { db }
    }
}

#[async_trait::async_trait]
impl Repository for SurrealLogRepository {
    type Entity = Log;
    type Id = LogId;

    async fn list(&self, method: ListMethod) -> RepositoryResult<Vec<Log>> {
        let result = match method {
            ListMethod::All => self.db.select("logs").await,
            ListMethod::Pagined { page, qtd } => {
                let mut response = self
                    .db
                    .query("SELECT * FROM logs LIMIT $qtd START $page")
                    .bind(("qtd", qtd))
                    .bind(("page", page))
                    .await
                    .map_err(|_| RepositoryError::DataError)?;

                response.take(0)
            }
        };

        match result {
            Ok(records_list) => {
                let records: Vec<LogRecord> = records_list;
                let list: Vec<Log> = records.iter().map(|record| record.to_entity()).collect();
                Ok(list)
            }
            Err(_) => Err(RepositoryError::DatabaseConnection),
        }
    }

    async fn find_by_id(&self, id: LogId) -> RepositoryResult<Log> {
        let record: Option<LogRecord> = self
            .db
            .select(("logs", id.0.clone()))
            .await
            .map_err(|_| RepositoryError::DatabaseConnection)?;

        match record {
            Some(r) => Ok(r.to_entity()),
            None => Err(RepositoryError::EntityNotFound(id.0.clone())),
        }
    }

    async fn create(&self, entity: Log) -> RepositoryResult<Log> {
        use surrealdb::types::RecordIdKey;

        let record_data = LogRecord {
            id: RecordId::new("logs", RecordIdKey::rand()),
            source: match entity.source {
                LogSource::Database => LogSourceRecord::Database,
                LogSource::Server => LogSourceRecord::Server,
            },
            action: entity.action,
            message: entity.message,
            table_affected: entity.table_affected,
            record_id: entity.record_id,
            timestamp: entity.timestamp,
        };

        let created: Option<LogRecord> = self
            .db
            .create("logs")
            .content(record_data)
            .await
            .map_err(|e| {
                println!("{e:?}");
                RepositoryError::DataError
            })?;

        Ok(created.unwrap().to_entity())
    }

    async fn update(&self, entity: Log) -> RepositoryResult<Log> {
        let id = entity.id.clone().ok_or(RepositoryError::DataError)?;

        let record_data = LogRecord {
            id: RecordId::new("logs", id.0.clone()),
            source: match entity.source {
                LogSource::Database => LogSourceRecord::Database,
                LogSource::Server => LogSourceRecord::Server,
            },
            action: entity.action,
            message: entity.message,
            table_affected: entity.table_affected,
            record_id: entity.record_id,
            timestamp: entity.timestamp,
        };

        let record: Option<LogRecord> = self
            .db
            .update(("logs", id.0.clone()))
            .content(record_data)
            .await
            .map_err(|_| RepositoryError::DataError)?;

        match record {
            Some(r) => Ok(r.to_entity()),
            None => Err(RepositoryError::EntityNotFound(id.0.clone())),
        }
    }

    async fn delete(&self, id: LogId) -> RepositoryResult<Log> {
        let record: Option<LogRecord> = self
            .db
            .delete(("logs", id.0.clone()))
            .await
            .map_err(|_| RepositoryError::DataError)?;

        match record {
            Some(r) => Ok(r.to_entity()),
            None => Err(RepositoryError::EntityNotFound(id.0.clone())),
        }
    }
}

#[async_trait::async_trait]
impl LogRepository for SurrealLogRepository {}
