use crate::Repository;
use ipanel_domain::models::domain::{Domain, DomainId};

#[async_trait::async_trait]
pub trait DomainRepository: Repository<Entity = Domain, Id = DomainId> {}
