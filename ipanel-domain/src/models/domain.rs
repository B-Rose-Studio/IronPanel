#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DomainId(pub String);

#[derive(Debug, Clone, PartialEq)]
pub struct Domain {
    pub id: Option<DomainId>,
    pub name: String,
    pub active: bool,
    pub url: Option<String>,
    pub user_service: Option<String>,
    pub password_user: Option<String>,
    pub base_dn: Option<String>,
    pub user_filter: Option<String>,
}
