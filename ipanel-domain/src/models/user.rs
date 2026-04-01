use super::{date::Weekday, domain::DomainId, group::GroupId};
use serde_json::Value;
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UserId(pub String);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UserType {
    Common,
    Admin,
}

impl UserType {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "common" => Some(UserType::Common),
            "admin" => Some(UserType::Admin),
            _ => None,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            UserType::Common => "common".to_string(),
            UserType::Admin => "admin".to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct User {
    pub id: Option<UserId>,
    pub username: String,
    pub active: bool,
    pub password: Option<String>,
    pub email: Option<String>,
    pub user_type: UserType,
    pub optional_data: BTreeMap<String, Value>,
    pub domain: Option<DomainId>,
    pub group: GroupId,
    pub weekdays: Vec<Weekday>,
}
