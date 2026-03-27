use crate::Repository;
use ipanel_domain::models::auth::{Auth, AuthId};

#[async_trait::async_trait]
pub trait AuthRepository: Repository<Entity = Auth, Id = AuthId> {}
