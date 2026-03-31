use crate::app::App;
use ipanel_repositories::surrealdb::{
    auth::SurrealAuthRepository, domain::SurrealDomainRepository, group::SurrealGroupRepository,
    job::SurrealJobRepository, log::SurrealLogRepository, user::SurrealUserRepository,
};
use ipanel_services::{
    auth::{GetAuthsService, impls::GetAuthsByUserAndDomain},
    database::{DatabaseProviderService, impls::SurrealDatabaseProvider},
};
use surrealdb::{Surreal, engine::remote::ws::Client};

mod actions;
mod app;

pub type DbClient = Surreal<Client>;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let app = App::new();

    app.register_service::<dyn GetAuthsService>(GetAuthsByUserAndDomain::new().build())
        .await;

    app.register_service::<dyn DatabaseProviderService<DbClient>>(
        SurrealDatabaseProvider::new("localhost:8000", "ipanel", "applications", "admin", "admin")
            .build(),
    )
    .await;

    let db_provider = app
        .get_service::<dyn DatabaseProviderService<DbClient>>()
        .await;

    let db = db_provider.run().await.unwrap();
    let _auth_repo = SurrealAuthRepository::new(db.clone());
    let _domain_repo = SurrealDomainRepository::new(db.clone());
    let _user_repo = SurrealUserRepository::new(db.clone());
    let _group_repo = SurrealGroupRepository::new(db.clone());
    let _job_repo = SurrealJobRepository::new(db.clone());
    let _log_repo = SurrealLogRepository::new(db.clone());

    app.run().await
}
