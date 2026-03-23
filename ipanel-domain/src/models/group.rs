use super::auth::AuthId;
use super::date::Weekday;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GroupId(pub String);

#[derive(Debug, Clone, PartialEq)]
pub struct Group {
    pub id: Option<GroupId>,
    pub name: String,
    pub description: Option<String>,
    pub default_auth_method: Option<AuthId>,
    pub weekdays: Vec<Weekday>,
}
