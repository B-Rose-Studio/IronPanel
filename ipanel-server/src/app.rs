use crate::actions::auth;
use actix_web::{App as ServerApp, HttpServer, web};
use ipanel_services::ServiceManager;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct App {
    services: Arc<ServiceManager>,
}

impl App {
    pub fn new() -> Self {
        Self {
            services: Arc::new(ServiceManager::new()),
        }
    }

    pub async fn run(&self) -> anyhow::Result<()> {
        let app_state = web::Data::new(self.clone());

        HttpServer::new(move || {
            ServerApp::new()
                .app_data(app_state.clone())
                .service(web::scope("/auth").route(
                    "/{username}",
                    web::get().to(
                        move |username: web::Path<String>, app_state: web::Data<App>| async move {
                            let services = app_state.services.clone();
                            auth::get_auth_methods(&services, &username)
                                .await
                                .join(" | ")
                        },
                    ),
                ))
        })
        .bind("0.0.0.0:8080")?
        .run()
        .await?;

        Ok(())
    }
}
