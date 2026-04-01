use crate::{Service, auth::AuthServiceError};
use ipanel_domain::models::{auth::Auth, user::UserId};
use ipanel_repositories::interfaces::auth::AuthRepository;
use std::sync::Arc;

#[async_trait::async_trait]
pub trait GetAuthsByUserService: Service {
    fn build(self) -> Arc<dyn GetAuthsByUserService>;
    async fn run(&self, user: UserId) -> Result<Vec<Auth>, AuthServiceError>;
}

pub use impls::*;
mod impls {
    use super::*;

    pub struct GetAuthsByUser {
        auth_repo: Arc<dyn AuthRepository>,
    }

    impl Service for GetAuthsByUser {}

    impl GetAuthsByUser {
        pub fn new(auth_repo: Arc<dyn AuthRepository>) -> Self {
            Self { auth_repo }
        }
    }

    #[async_trait::async_trait]
    impl GetAuthsByUserService for GetAuthsByUser {
        fn build(self) -> Arc<dyn GetAuthsByUserService> {
            Arc::new(self)
        }

        async fn run(&self, user: UserId) -> Result<Vec<Auth>, AuthServiceError> {
            let list = self
                .auth_repo
                .list_auths_by_user(user)
                .await
                .map_err(|_| AuthServiceError::Unknow)?;

            Ok(list)
        }
    }
}
