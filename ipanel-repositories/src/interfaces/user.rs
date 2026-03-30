use crate::Repository;
use ipanel_domain::models::user::{User, UserId};

#[async_trait::async_trait]
pub trait UserRepository: Repository<Entity = User, Id = UserId> {}
