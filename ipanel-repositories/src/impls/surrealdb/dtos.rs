use ipanel_domain::models::auth::{Auth, AuthId};
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
