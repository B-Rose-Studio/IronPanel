mod get_auths;
pub use get_auths::*;

use crate::ServiceError;

#[derive(Debug)]
pub struct AuthServiceError {
    code: String,
    description: String,
    content: String,
}

impl AuthServiceError {
    pub fn unknown() -> Self {
        Self {
            code: "unknown".to_string(),
            description: "Unknown error".to_string(),
            content: "Unknown error".to_string(),
        }
    }
}

impl ServiceError for AuthServiceError {
    fn code(&self) -> String {
        self.code.clone()
    }

    fn content(&self) -> &impl serde::Serialize {
        &self.content
    }

    fn description(&self) -> String {
        self.description.clone()
    }
}
