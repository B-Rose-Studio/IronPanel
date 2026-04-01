use super::{date::Weekday, domain::DomainId, group::GroupId};
use serde_json::Value;
use std::{collections::BTreeMap, fmt::Display, str::FromStr};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UserId(pub String);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UserType {
    Common,
    Admin,
}

impl Display for UserType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserType::Common => write!(f, "common"),
            UserType::Admin => write!(f, "admin"),
        }
    }
}

impl FromStr for UserType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "common" => Ok(UserType::Common),
            "admin" => Ok(UserType::Admin),
            _ => Err(()),
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
