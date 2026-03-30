use crate::app::App;
use ipanel_repositories::{
    DBClient,
    surrealdb::{
        auth::SurrealAuthRepository, domain::SurrealDomainRepository, job::SurrealJobRepository,
        user::SurrealUserRepository,
    },
};
use ipanel_services::auth::{GetAuthsService, impls::GetAuthsByUserAndDomain};
use surrealdb::{Surreal, engine::remote::ws::Ws, opt::auth::Root};

mod actions;
mod app;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let db = Surreal::new::<Ws>("localhost:8000").await.unwrap();

    let _ = db
        .signin(Root {
            username: "admin".into(),
            password: "admin".into(),
        })
        .await;

    let _ = db.use_ns("applications").use_db("ipanel").await;

    let db = DBClient::new(db);
    let _auth_repo = SurrealAuthRepository::new(db.clone());
    let _domain_repo = SurrealDomainRepository::new(db.clone());
    let _user_repo = SurrealUserRepository::new(db.clone());
    let _job_repo = SurrealJobRepository::new(db.clone());

    let app = App::new();

    app.register_service(GetAuthsByUserAndDomain {}.into_service())
        .await;

    let _ = app.get_service::<dyn GetAuthsService>().await;

    app.run().await
}
