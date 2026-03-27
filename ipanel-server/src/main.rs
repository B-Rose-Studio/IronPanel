use crate::app::App;
use ipanel_repositories::auth::surrealdb::SurrealAuthRepository;
use ipanel_services::auth::{GetAuthsService, impls::GetAuthsByUserAndDomain};
use std::sync::Arc;
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

    let db = Arc::new(db);
    let _auth_repo = SurrealAuthRepository { db };

    let app = App::new();

    app.register_service(GetAuthsByUserAndDomain {}.into_service())
        .await;

    let _ = app.get_service::<dyn GetAuthsService>().await;

    app.run().await
}
