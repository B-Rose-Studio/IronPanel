use crate::Repository;
use ipanel_domain::models::group::{Group, GroupId};

#[async_trait::async_trait]
pub trait GroupRepository: Send + Sync + Repository<Entity = Group, Id = GroupId> {}
