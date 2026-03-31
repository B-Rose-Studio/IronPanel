use serde::Serialize;
use std::{
    any::{Any, TypeId},
    collections::HashMap,
    fmt::Debug,
    sync::Arc,
};
use tokio::sync::RwLock;

pub mod auth;
pub mod database;

pub trait Service: Sync + Send {}
impl<T: ?Sized + Service> Service for Arc<T> {}

#[derive(Debug, Default)]
pub struct ServiceManager {
    map: RwLock<HashMap<TypeId, Arc<dyn Any + Sync + Send>>>,
}

impl ServiceManager {
    pub fn new() -> Self {
        Self {
            map: RwLock::new(HashMap::new()),
        }
    }

    pub async fn register<S: Service + 'static>(&self, service: S) {
        let mut map = self.map.write().await;
        map.insert(TypeId::of::<S>(), Arc::new(service));
    }

    pub async fn get<S: Service + 'static>(&self) -> Option<Arc<S>> {
        let map = self.map.read().await;

        if let Some(service_any) = map.get(&TypeId::of::<S>()) {
            let s = service_any.clone();
            return s.downcast().ok();
        }

        None
    }
}

pub trait ServiceError: Debug {
    fn code(&self) -> String;
    fn description(&self) -> String;
    fn content(&self) -> &impl Serialize;
}
