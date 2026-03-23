use crate::models::group::GroupId;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ConfigId(pub String);

#[derive(Debug, Clone, PartialEq)]
pub struct ApplicationConfig {
    pub id: Option<ConfigId>,
    pub auto_registre: bool,
    pub default_group: GroupId,
}
