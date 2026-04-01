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
                "/{domain}/{username}",
                web::get().to(
                    move |path: web::Path<(String, String)>, app_state: web::Data<App>| async move {
                        let services = app_state.services.clone();
                        let (domain, username) = path.into_inner();
                        auth::get_auth_methods(&services, &domain, &username)
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

impl App {
    pub async fn register_service<T>(&self, service: Arc<T>)
    where
        T: ipanel_services::Service + ?Sized + 'static,
        Arc<T>: ipanel_services::Service,
    {
        self.services.register(service).await;
    }

    #[allow(dead_code)]
    pub async fn get_service<T>(&self) -> Arc<Arc<T>>
    where
        T: ipanel_services::Service + ?Sized + 'static,
        Arc<T>: ipanel_services::Service,
    {
        self.services.get::<Arc<T>>().await.unwrap()
    }
}
