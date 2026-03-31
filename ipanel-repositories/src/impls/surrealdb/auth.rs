use crate::{
    DBClient, ListMethod, Repository, RepositoryError, RepositoryResult,
    interfaces::auth::AuthRepository,
    surrealdb::dtos::auth::{AuthRecord, UserAuthRecord},
};
use ipanel_domain::models::{
    auth::{Auth, AuthId, UserAuth, UserAuthId},
    user::UserId,
};
use surrealdb::{
    Surreal,
    engine::remote::ws::Client,
    types::{RecordId, RecordIdKey},
};

pub struct SurrealAuthRepository {
    db: DBClient<Surreal<Client>>,
}

impl SurrealAuthRepository {
    pub fn new(db: DBClient<Surreal<Client>>) -> Self {
        Self { db }
    }
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

#[async_trait::async_trait]
impl AuthRepository for SurrealAuthRepository {
    async fn assign_auth_to_user(&self, user_auth: UserAuth) -> RepositoryResult<UserAuth> {
        let record_data = UserAuthRecord {
            id: RecordId::new("user_auth", RecordIdKey::rand()),
            r#in: RecordId::new("users", user_auth.user_id.0.clone()),
            out: RecordId::new("auths", user_auth.auth_id.0.clone()),
            params_values: user_auth.params_values,
        };

        let created: Vec<UserAuthRecord> = self
            .db
            .insert("user_auth")
            .relation(record_data)
            .await
            .map_err(|e| {
                println!("{e:?}");
                RepositoryError::DataError
            })?;

        Ok(created.first().unwrap().to_entity())
    }

    async fn get_user_auth(&self, id: UserAuthId) -> RepositoryResult<UserAuth> {
        let record: Option<UserAuthRecord> = self
            .db
            .select(("user_auth", id.0.clone()))
            .await
            .map_err(|_| RepositoryError::DatabaseConnection)?;

        match record {
            Some(r) => Ok(r.to_entity()),
            None => Err(RepositoryError::EntityNotFound(id.0.clone())),
        }
    }

    async fn list_auths_by_user(&self, user_id: UserId) -> RepositoryResult<Vec<UserAuth>> {
        let target_user = RecordId::new("users", user_id.0);

        let mut response = self
            .db
            .query("SELECT * FROM user_auth WHERE in = $target")
            .bind(("target", target_user))
            .await
            .map_err(|_| RepositoryError::DataError)?;

        let records: Vec<UserAuthRecord> =
            response.take(0).map_err(|_| RepositoryError::DataError)?;
        Ok(records.iter().map(|record| record.to_entity()).collect())
    }

    async fn list_users_by_auth(&self, auth_id: AuthId) -> RepositoryResult<Vec<UserAuth>> {
        let target_auth = RecordId::new("auths", auth_id.0);

        let mut response = self
            .db
            .query("SELECT * FROM user_auth WHERE out = $target")
            .bind(("target", target_auth))
            .await
            .map_err(|_| RepositoryError::DataError)?;

        let records: Vec<UserAuthRecord> =
            response.take(0).map_err(|_| RepositoryError::DataError)?;
        Ok(records.iter().map(|record| record.to_entity()).collect())
    }

    async fn update_user_auth(&self, user_auth: UserAuth) -> RepositoryResult<UserAuth> {
        let id = user_auth.id.clone().ok_or(RepositoryError::DataError)?;

        let record_data = UserAuthRecord {
            id: RecordId::new("user_auth", id.0.clone()),
            r#in: RecordId::new("users", user_auth.user_id.0.clone()),
            out: RecordId::new("auths", user_auth.auth_id.0.clone()),
            params_values: user_auth.params_values,
        };

        let record: Option<UserAuthRecord> = self
            .db
            .update(("user_auth", id.0.clone()))
            .content(record_data)
            .await
            .map_err(|_| RepositoryError::DataError)?;

        match record {
            Some(r) => Ok(r.to_entity()),
            None => Err(RepositoryError::EntityNotFound(id.0.clone())),
        }
    }

    async fn remove_auth_from_user(&self, id: UserAuthId) -> RepositoryResult<UserAuth> {
        let record: Option<UserAuthRecord> = self
            .db
            .delete(("user_auth", id.0.clone()))
            .await
            .map_err(|_| RepositoryError::DataError)?;

        match record {
            Some(r) => Ok(r.to_entity()),
            None => Err(RepositoryError::EntityNotFound(id.0.clone())),
        }
    }
}
