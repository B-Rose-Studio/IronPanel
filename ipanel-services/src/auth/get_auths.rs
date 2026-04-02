use crate::{Service, auth::AuthServiceError};
use ipanel_domain::models::{
    auth::{Auth, AuthId},
    user::UserId,
};
use ipanel_repositories::interfaces::auth::AuthRepository;
use std::sync::Arc;

pub enum Filter {
    Id(AuthId),
    ByUser(UserId),
}

#[async_trait::async_trait]
pub trait GetAuthsService: Service {
    fn build(self) -> Arc<dyn GetAuthsService>;
    async fn run(&self, filter: Filter) -> Result<Vec<Auth>, AuthServiceError>;
}

pub use impls::*;
mod impls {
    use super::*;

    pub struct GetAuths {
        auth_repo: Arc<dyn AuthRepository>,
    }

    impl Service for GetAuths {}

    impl GetAuths {
        pub fn new(auth_repo: Arc<dyn AuthRepository>) -> Self {
            Self { auth_repo }
        }
    }

    #[async_trait::async_trait]
    impl GetAuthsService for GetAuths {
        fn build(self) -> Arc<dyn GetAuthsService> {
            Arc::new(self)
        }

        async fn run(&self, filter: Filter) -> Result<Vec<Auth>, AuthServiceError> {
            match filter {
                Filter::Id(id) => {
                    let auth = self
                        .auth_repo
                        .find_by_id(id)
                        .await
                        .map_err(|_| AuthServiceError::unknown())?;

                    Ok(vec![auth])
                }

                Filter::ByUser(id) => {
                    let list = self
                        .auth_repo
                        .list_auths_by_user(id)
                        .await
                        .map_err(|_| AuthServiceError::unknown())?;

                    Ok(list)
                } //_ => Err(AuthServiceError::Unknow),
            }
        }
    }
}
