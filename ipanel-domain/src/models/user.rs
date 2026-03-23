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
