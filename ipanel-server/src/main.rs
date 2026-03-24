use crate::app::App;

mod actions;
mod app;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let app = App::new();

    app.run().await
}
