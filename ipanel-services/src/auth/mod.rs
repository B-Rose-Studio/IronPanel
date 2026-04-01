mod get_auths_by_user;
pub use get_auths_by_user::*;

use crate::ServiceError;

#[derive(Debug)]
pub enum AuthServiceError {
    Unknow,
}

impl ServiceError for AuthServiceError {
    fn code(&self) -> String {
        todo!()
    }

    fn content(&self) -> &impl serde::Serialize {
        &()
    }

    fn description(&self) -> String {
        todo!()
    }
}
