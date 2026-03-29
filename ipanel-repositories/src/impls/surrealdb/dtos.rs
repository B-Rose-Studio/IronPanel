use ipanel_domain::models::{
    auth::{Auth, AuthId},
    domain::{Domain, DomainId},
};
use std::collections::BTreeMap;
use surrealdb::types::{RecordId, SurrealValue};

#[derive(SurrealValue)]
pub struct AuthRecord {
    pub id: RecordId,
    pub name: String,
    pub params: BTreeMap<String, String>,
}

impl AuthRecord {
    pub fn to_entity(&self) -> Auth {
        Auth {
            id: Some(AuthId(
                self.id.clone().key.into_value().into_string().unwrap(),
            )),
            name: self.name.clone(),
            params: self.params.clone(),
        }
    }
}

#[derive(SurrealValue)]
pub struct DomainRecord {
    pub id: RecordId,
    pub name: String,
    pub active: bool,
    pub url: Option<String>,
    pub user_service: Option<String>,
    pub password_user: Option<String>,
    pub base_dn: Option<String>,
    pub user_filter: Option<String>,
}

impl DomainRecord {
    pub fn to_entity(&self) -> Domain {
        Domain {
            id: Some(DomainId(
                self.id.clone().key.into_value().into_string().unwrap(),
            )),
            name: self.name.clone(),
            active: self.active,
            url: self.url.clone(),
            user_service: self.user_service.clone(),
            password_user: self.password_user.clone(),
            base_dn: self.base_dn.clone(),
            user_filter: self.user_filter.clone(),
        }
    }
}
