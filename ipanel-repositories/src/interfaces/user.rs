use crate::{Repository, RepositoryResult};
use ipanel_domain::models::user::{User, UserId};

#[async_trait::async_trait]
pub trait UserRepository: Repository<Entity = User, Id = UserId> {
    async fn find_by_username_and_domain(
        &self,
        username: String,
        domain: String,
    ) -> RepositoryResult<User>;
}
