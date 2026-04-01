use crate::{
    DBClient, ListMethod, Repository, RepositoryError, RepositoryResult,
    interfaces::user::UserRepository,
    surrealdb::dtos::{WeekdayNameRecord, WeekdayRecord, user::UserRecord},
};
use ipanel_domain::models::{
    date::WeekdayName,
    user::{User, UserId},
};
use surrealdb::{Surreal, engine::any::Any, types::RecordId};

pub struct SurrealUserRepository {
    db: DBClient<Surreal<Any>>,
}

impl SurrealUserRepository {
    pub fn new(db: DBClient<Surreal<Any>>) -> Self {
        Self { db }
    }
}

#[async_trait::async_trait]
impl Repository for SurrealUserRepository {
    type Entity = User;
    type Id = UserId;

    async fn list(&self, method: ListMethod) -> RepositoryResult<Vec<User>> {
        let result = match method {
            ListMethod::All => self.db.select("users").await,
            ListMethod::Pagined { page, qtd } => {
                let mut response = self
                    .db
                    .query("SELECT * FROM users LIMIT $qtd START $page")
                    .bind(("qtd", qtd))
                    .bind(("page", page))
                    .await
                    .map_err(|_| RepositoryError::DataError)?;

                response.take(0)
            }
        };

        match result {
            Ok(records_list) => {
                let records: Vec<UserRecord> = records_list;
                let list: Vec<User> = records.iter().map(|record| record.to_entity()).collect();
                Ok(list)
            }
            Err(_) => Err(RepositoryError::DatabaseConnection),
        }
    }

    async fn find_by_id(&self, id: UserId) -> RepositoryResult<User> {
        let record: Option<UserRecord> = self
            .db
            .select(("users", id.0.clone()))
            .await
            .map_err(|_| RepositoryError::DatabaseConnection)?;

        match record {
            Some(r) => Ok(r.to_entity()),
            None => Err(RepositoryError::EntityNotFound(id.0.clone())),
        }
    }

    async fn create(&self, entity: User) -> RepositoryResult<User> {
        use surrealdb::types::RecordIdKey;

        let record_data = UserRecord {
            id: RecordId::new("users", RecordIdKey::rand()),
            username: entity.username,
            active: entity.active,
            password: entity.password,
            email: entity.email,
            r#type: entity.user_type.to_string(),
            optional_data: entity.optional_data,
            domain: entity.domain.map(|d| RecordId::new("domains", d.0)),
            group: RecordId::new("groups", entity.group.0),
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

        let created: Option<UserRecord> = self
            .db
            .create("users")
            .content(record_data)
            .await
            .map_err(|e| {
                println!("{e:?}");
                RepositoryError::DataError
            })?;

        Ok(created.unwrap().to_entity())
    }

    async fn update(&self, entity: User) -> RepositoryResult<User> {
        let id = entity.id.clone().ok_or(RepositoryError::DataError)?;

        let record_data = UserRecord {
            id: RecordId::new("users", id.0.clone()),
            username: entity.username,
            active: entity.active,
            password: entity.password,
            email: entity.email,
            r#type: entity.user_type.to_string(),
            optional_data: entity.optional_data,
            domain: entity.domain.map(|d| RecordId::new("domains", d.0)),
            group: RecordId::new("groups", entity.group.0),
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

        let record: Option<UserRecord> = self
            .db
            .update(("users", id.0.clone()))
            .content(record_data)
            .await
            .map_err(|_| RepositoryError::DataError)?;

        match record {
            Some(r) => Ok(r.to_entity()),
            None => Err(RepositoryError::EntityNotFound(id.0.clone())),
        }
    }

    async fn delete(&self, id: UserId) -> RepositoryResult<User> {
        let record: Option<UserRecord> = self
            .db
            .delete(("users", id.0.clone()))
            .await
            .map_err(|_| RepositoryError::DataError)?;

        match record {
            Some(r) => Ok(r.to_entity()),
            None => Err(RepositoryError::EntityNotFound(id.0.clone())),
        }
    }
}

#[async_trait::async_trait]
impl UserRepository for SurrealUserRepository {
    async fn find_by_username_and_domain(
        &self,
        username: String,
        domain: String,
    ) -> RepositoryResult<User> {
        let mut index_result = self
            .db
            .query("SELECT * FROM users WHERE username = $username AND domain.name = $domain")
            .bind(("username".to_string(), username.clone()))
            .bind(("domain".to_string(), domain.clone()))
            .await
            .map_err(|_| RepositoryError::DataError)?;

        let user: UserRecord = index_result
            .take::<Option<UserRecord>>(0)
            .map_err(|e| {
                println!("{e:?}");
                RepositoryError::DataError
            })?
            .ok_or(RepositoryError::EntityNotFound(
                "User not found".to_string(),
            ))?;

        Ok(user.to_entity())
    }
}
