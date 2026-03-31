use crate::app::App;
use ipanel_repositories::surrealdb::{
    auth::SurrealAuthRepository, domain::SurrealDomainRepository, group::SurrealGroupRepository,
    job::SurrealJobRepository, log::SurrealLogRepository, user::SurrealUserRepository,
};
use ipanel_services::{
    auth::{GetAuthsService, impls::GetAuthsByUserAndDomain},
    config::{AppConfigProviderService, impls::SurrealAppConfigProvider},
    database::{DatabaseProviderService, impls::SurrealDatabaseProvider},
};
use surrealdb::{Surreal, engine::remote::ws::Client};

mod actions;
mod app;

pub type DbClient = Surreal<Client>;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let app = App::new();

    let db_provider_service =
        SurrealDatabaseProvider::new("localhost:8000", "ipanel", "applications", "admin", "admin")
            .build();

    let get_auths_service = GetAuthsByUserAndDomain::new().build();

    let db = db_provider_service.run().await.unwrap();

    let app_config_provider_service = SurrealAppConfigProvider::new(db.clone()).build();

    let _auth_repo = SurrealAuthRepository::new(db.clone());
    let _domain_repo = SurrealDomainRepository::new(db.clone());
    let _user_repo = SurrealUserRepository::new(db.clone());
    let _group_repo = SurrealGroupRepository::new(db.clone());
    let _job_repo = SurrealJobRepository::new(db.clone());
    let _log_repo = SurrealLogRepository::new(db.clone());

    app.register_service::<dyn GetAuthsService>(get_auths_service)
        .await;

    app.register_service::<dyn DatabaseProviderService<DbClient>>(db_provider_service)
        .await;

    app.register_service::<dyn AppConfigProviderService>(app_config_provider_service)
        .await;

    app.run().await
}
