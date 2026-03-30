use crate::{
    DBClient, ListMethod, Repository, RepositoryError, RepositoryResult,
    interfaces::group::GroupRepository,
    surrealdb::dtos::{WeekdayNameRecord, WeekdayRecord, group::GroupRecord},
};
use ipanel_domain::models::{
    date::WeekdayName, // Ajuste o caminho se WeekdayName vier de outro arquivo
    group::{Group, GroupId},
};
use surrealdb::{Surreal, engine::remote::ws::Client, types::RecordId};

pub struct SurrealGroupRepository {
    db: DBClient<Surreal<Client>>,
}

impl SurrealGroupRepository {
    pub fn new(db: DBClient<Surreal<Client>>) -> Self {
        Self { db }
    }
}

#[async_trait::async_trait]
impl Repository for SurrealGroupRepository {
    type Entity = Group;
    type Id = GroupId;

    async fn list(&self, method: ListMethod) -> RepositoryResult<Vec<Group>> {
        let result = match method {
            ListMethod::All => self.db.select("groups").await,
            ListMethod::Pagined { page, qtd } => {
                let mut response = self
                    .db
                    .query("SELECT * FROM groups LIMIT $qtd START $page")
                    .bind(("qtd", qtd))
                    .bind(("page", page))
                    .await
                    .map_err(|_| RepositoryError::DataError)?;

                response.take(0)
            }
        };

        match result {
            Ok(records_list) => {
                let records: Vec<GroupRecord> = records_list;
                let list: Vec<Group> = records.iter().map(|record| record.to_entity()).collect();
                Ok(list)
            }
            Err(_) => Err(RepositoryError::DatabaseConnection),
        }
    }

    async fn find_by_id(&self, id: GroupId) -> RepositoryResult<Group> {
        let record: Option<GroupRecord> = self
            .db
            .select(("groups", id.0.clone()))
            .await
            .map_err(|_| RepositoryError::DatabaseConnection)?;

        match record {
            Some(r) => Ok(r.to_entity()),
            None => Err(RepositoryError::EntityNotFound(id.0.clone())),
        }
    }

    async fn create(&self, entity: Group) -> RepositoryResult<Group> {
        use surrealdb::types::RecordIdKey;

        let record_data = GroupRecord {
            id: RecordId::new("groups", RecordIdKey::rand()),
            name: entity.name,
            description: entity.description,
            // Converte a referência do domínio para o RecordId do banco
            default_auth_method: entity
                .default_auth_method
                .map(|auth_id| RecordId::new("auths", auth_id.0)),
            // Converte as structs aninhadas
            weekdays: entity
                .weekdays
                .into_iter()
                .map(|w| WeekdayRecord {
                    day: match w.day {
                        WeekdayName::Mon => WeekdayNameRecord::Mon,
                        WeekdayName::Tue => WeekdayNameRecord::Tue,
                        WeekdayName::Wed => WeekdayNameRecord::Wed,
                        WeekdayName::Thu => WeekdayNameRecord::Thu,
                        WeekdayName::Fri => WeekdayNameRecord::Fri,
                        WeekdayName::Sat => WeekdayNameRecord::Sat,
                        WeekdayName::Sun => WeekdayNameRecord::Sun,
                    },
                    start_time: w.start_time,
                    end_time: w.end_time,
                })
                .collect(),
        };

        let created: Option<GroupRecord> = self
            .db
            .create("groups")
            .content(record_data)
            .await
            .map_err(|e| {
                println!("{e:?}");
                RepositoryError::DataError
            })?;

        Ok(created.unwrap().to_entity())
    }

    async fn update(&self, entity: Group) -> RepositoryResult<Group> {
        let id = entity.id.clone().ok_or(RepositoryError::DataError)?;

        let record_data = GroupRecord {
            id: RecordId::new("groups", id.0.clone()),
            name: entity.name,
            description: entity.description,
            default_auth_method: entity
                .default_auth_method
                .map(|auth_id| RecordId::new("auths", auth_id.0)),
            weekdays: entity
                .weekdays
                .into_iter()
                .map(|w| WeekdayRecord {
                    day: match w.day {
                        WeekdayName::Mon => WeekdayNameRecord::Mon,
                        WeekdayName::Tue => WeekdayNameRecord::Tue,
                        WeekdayName::Wed => WeekdayNameRecord::Wed,
                        WeekdayName::Thu => WeekdayNameRecord::Thu,
                        WeekdayName::Fri => WeekdayNameRecord::Fri,
                        WeekdayName::Sat => WeekdayNameRecord::Sat,
                        WeekdayName::Sun => WeekdayNameRecord::Sun,
                    },
                    start_time: w.start_time,
                    end_time: w.end_time,
                })
                .collect(),
        };

        let record: Option<GroupRecord> = self
            .db
            .update(("groups", id.0.clone()))
            .content(record_data)
            .await
            .map_err(|_| RepositoryError::DataError)?;

        match record {
            Some(r) => Ok(r.to_entity()),
            None => Err(RepositoryError::EntityNotFound(id.0.clone())),
        }
    }

    async fn delete(&self, id: GroupId) -> RepositoryResult<Group> {
        let record: Option<GroupRecord> = self
            .db
            .delete(("groups", id.0.clone()))
            .await
            .map_err(|_| RepositoryError::DataError)?;

        match record {
            Some(r) => Ok(r.to_entity()),
            None => Err(RepositoryError::EntityNotFound(id.0.clone())),
        }
    }
}

#[async_trait::async_trait]
impl GroupRepository for SurrealGroupRepository {}
