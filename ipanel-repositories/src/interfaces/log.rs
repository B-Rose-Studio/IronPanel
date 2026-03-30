use crate::Repository;
use ipanel_domain::models::log::{Log, LogId};

#[async_trait::async_trait]
pub trait LogRepository: Repository<Entity = Log, Id = LogId> {}
