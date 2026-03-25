use crate::app::App;
use ipanel_services::auth::{GetAuthsService, impls::GetAuthsByUserAndDomain};

mod actions;
mod app;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let app = App::new();

    app.register_service(GetAuthsByUserAndDomain {}.into_service())
        .await;

    app.run().await
}
