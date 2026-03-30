use super::user::UserId;
use serde_json::Value;
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AuthId(pub String);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UserAuthId(pub String);

#[derive(Debug, Clone, PartialEq)]
pub struct Auth {
    pub id: Option<AuthId>,
    pub name: String,
    pub params: BTreeMap<String, String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct UserAuth {
    pub id: Option<UserAuthId>,
    pub user_id: UserId,
    pub auth_id: AuthId,
    pub params_values: BTreeMap<String, Value>,
}
