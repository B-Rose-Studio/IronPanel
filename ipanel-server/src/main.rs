use crate::app::App;
use ipanel_repositories::surrealdb::{
    auth::SurrealAuthRepository, domain::SurrealDomainRepository, group::SurrealGroupRepository,
    job::SurrealJobRepository, log::SurrealLogRepository, user::SurrealUserRepository,
};
use ipanel_services::{
    auth::{GetAuthsByUser, GetAuthsByUserService},
    config::{AppConfigProviderService, SurrealAppConfigProvider},
    database::{DBConfig, DatabaseProviderService, SurrealDatabaseProvider},
};
use std::sync::Arc;
use surrealdb::{Surreal, engine::any::Any};

mod actions;
mod app;

pub type DbClient = Surreal<Any>;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv()?;
    let app = App::new();

    let db_provider_service = SurrealDatabaseProvider::new(DBConfig::Env).build();
    let db = db_provider_service.run().await.unwrap();

    let app_config_provider_service = SurrealAppConfigProvider::new(db.clone()).build();

    let auth_repo = Arc::new(SurrealAuthRepository::new(db.clone()));
    let _domain_repo = Arc::new(SurrealDomainRepository::new(db.clone()));
    let _user_repo = Arc::new(SurrealUserRepository::new(db.clone()));
    let _group_repo = Arc::new(SurrealGroupRepository::new(db.clone()));
    let _job_repo = Arc::new(SurrealJobRepository::new(db.clone()));
    let _log_repo = Arc::new(SurrealLogRepository::new(db.clone()));

    let get_auths_by_user_service = GetAuthsByUser::new(auth_repo.clone()).build();

    app.register_service::<dyn GetAuthsByUserService>(get_auths_by_user_service)
        .await;

    app.register_service::<dyn DatabaseProviderService<DbClient>>(db_provider_service)
        .await;

    app.register_service::<dyn AppConfigProviderService>(app_config_provider_service)
        .await;

    app.run().await
}
