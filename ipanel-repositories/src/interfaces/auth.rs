use crate::{Repository, RepositoryResult};
use ipanel_domain::models::{
    auth::{Auth, AuthId, UserAuth, UserAuthId},
    user::UserId,
};

#[async_trait::async_trait]
pub trait AuthRepository: Send + Sync + Repository<Entity = Auth, Id = AuthId> {
    async fn assign_auth_to_user(&self, user_auth: UserAuth) -> RepositoryResult<UserAuth>;

    async fn get_user_auth(&self, id: UserAuthId) -> RepositoryResult<UserAuth>;

    async fn list_userauths_by_user(&self, user_id: UserId) -> RepositoryResult<Vec<UserAuth>>;

    async fn list_userauths_by_auth(&self, auth_id: AuthId) -> RepositoryResult<Vec<UserAuth>>;

    async fn list_auths_by_user(&self, user_id: UserId) -> RepositoryResult<Vec<Auth>>;

    async fn update_user_auth(&self, user_auth: UserAuth) -> RepositoryResult<UserAuth>;

    async fn remove_auth_from_user(&self, id: UserAuthId) -> RepositoryResult<UserAuth>;
}
