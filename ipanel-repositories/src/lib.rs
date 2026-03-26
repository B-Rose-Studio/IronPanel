use std::fmt::Debug;
use thiserror::Error;

pub mod auth;

pub type RepositoryResult<T> = Result<T, RepositoryError>;

#[async_trait::async_trait]
pub trait Repository {
    type Entity;
    type Id;

    async fn list(&self, method: ListMethod) -> RepositoryResult<Vec<Self::Entity>>;

    async fn find_by_id(&self, id: Self::Id) -> RepositoryResult<Self::Entity>;

    async fn create(&self, entity: Self::Entity) -> RepositoryResult<Self::Entity>;

    async fn update(&self, entity: Self::Entity) -> RepositoryResult<Self::Entity>;

    async fn delete(&self, id: Self::Id) -> RepositoryResult<Self::Entity>;
}

#[derive(Error, Debug)]
pub enum RepositoryError {
    #[error("The entity '{0}' not found")]
    EntityNotFound(String),

    #[error("Database not responde")]
    DatabaseConnection,

    #[error("Invalid query send to database")]
    DataError,

    #[error("Unknown error repository")]
    Unknow,
}

pub enum ListMethod {
    All,
    Pagined { page: u32, qtd: u32 },
}
