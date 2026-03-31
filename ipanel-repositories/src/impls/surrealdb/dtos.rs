use chrono::{DateTime, Utc};
use ipanel_domain::models::auth::{Auth, AuthId};
use ipanel_domain::models::auth::{UserAuth, UserAuthId};
use ipanel_domain::models::date::{Weekday, WeekdayName};
use ipanel_domain::models::domain::{Domain, DomainId};
use ipanel_domain::models::group::{Group, GroupId};
use ipanel_domain::models::job::AssigneeId;
use ipanel_domain::models::job::JobTrigger;
use ipanel_domain::models::job::{AssignedJob, AssignedJobId};
use ipanel_domain::models::job::{Job, JobId};
use ipanel_domain::models::log::LogSource;
use ipanel_domain::models::log::{Log, LogId};
use ipanel_domain::models::user::UserType;
use ipanel_domain::models::user::{User, UserId};
use serde_json::Value;
use std::collections::BTreeMap;
use surrealdb::types::{RecordId, SurrealValue};

#[derive(SurrealValue, Clone)]
pub enum WeekdayNameRecord {
    Mon,
    Tue,
    Wed,
    Thu,
    Fri,
    Sat,
    Sun,
}

impl WeekdayNameRecord {
    pub fn to_entity(&self) -> WeekdayName {
        match self {
            Self::Mon => WeekdayName::Mon,
            Self::Tue => WeekdayName::Tue,
            Self::Wed => WeekdayName::Wed,
            Self::Thu => WeekdayName::Thu,
            Self::Fri => WeekdayName::Fri,
            Self::Sat => WeekdayName::Sat,
            Self::Sun => WeekdayName::Sun,
        }
    }
}

#[derive(SurrealValue, Clone)]
pub struct WeekdayRecord {
    pub day: WeekdayNameRecord,
    pub start_time: String,
    pub end_time: String,
}

impl WeekdayRecord {
    pub fn to_entity(&self) -> Weekday {
        Weekday {
            day: self.day.to_entity(),
            start_time: self.start_time.clone(),
            end_time: self.end_time.clone(),
        }
    }
}

#[allow(dead_code)]
pub mod auth {
    use super::*;

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
    pub struct UserAuthRecord {
        pub id: RecordId,
        #[surreal(rename = "in")]
        pub r#in: RecordId,
        pub out: RecordId,
        pub params_values: BTreeMap<String, Value>,
    }

    impl UserAuthRecord {
        pub fn to_entity(&self) -> UserAuth {
            UserAuth {
                id: Some(UserAuthId(
                    self.id.clone().key.into_value().into_string().unwrap(),
                )),
                user_id: UserId(self.r#in.clone().key.into_value().into_string().unwrap()),
                auth_id: AuthId(self.out.clone().key.into_value().into_string().unwrap()),
                params_values: self.params_values.clone(),
            }
        }
    }
}

pub mod domain {
    use super::*;

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
}

pub mod group {
    use super::*;

    #[derive(SurrealValue)]
    pub struct GroupRecord {
        pub id: RecordId,
        pub name: String,
        pub description: Option<String>,
        pub default_auth_method: Option<RecordId>,
        pub weekdays: Vec<WeekdayRecord>,
    }

    impl GroupRecord {
        pub fn to_entity(&self) -> Group {
            Group {
                id: Some(GroupId(
                    self.id.clone().key.into_value().into_string().unwrap(),
                )),
                name: self.name.clone(),
                description: self.description.clone(),
                default_auth_method: self
                    .default_auth_method
                    .as_ref()
                    .map(|rid| AuthId(rid.clone().key.into_value().into_string().unwrap())),

                weekdays: self.weekdays.iter().map(|w| w.to_entity()).collect(),
            }
        }
    }
}

#[allow(dead_code)]
pub mod job {
    use super::*;

    #[derive(SurrealValue, Clone)]
    pub enum JobTriggerRecord {
        Manual,
        OnStartup,
        OnLogout,
        Interval { seconds: u64 },
        Cron { expression: String },
        SpecificDate { date: DateTime<Utc> },
    }

    impl JobTriggerRecord {
        pub fn to_entity(&self) -> JobTrigger {
            match self {
                Self::Manual => JobTrigger::Manual,
                Self::OnStartup => JobTrigger::OnStartup,
                Self::OnLogout => JobTrigger::OnLogout,
                Self::Interval { seconds } => JobTrigger::Interval { seconds: *seconds },
                Self::Cron { expression } => JobTrigger::Cron {
                    expression: expression.clone(),
                },
                Self::SpecificDate { date } => JobTrigger::SpecificDate { date: *date },
            }
        }
    }

    #[derive(SurrealValue)]
    pub struct JobRecord {
        pub id: RecordId,
        pub name: String,
        pub description: Option<String>,
        pub content: String,
        pub params: BTreeMap<String, Value>,
    }

    impl JobRecord {
        pub fn to_entity(&self) -> Job {
            Job {
                id: Some(JobId(
                    self.id.clone().key.into_value().into_string().unwrap(),
                )),
                name: self.name.clone(),
                description: self.description.clone(),
                content: self.content.clone(),
                params: self.params.clone(),
            }
        }
    }

    #[derive(SurrealValue, Clone)]
    pub enum AssigneeIdRecord {
        User(RecordId),
        Group(RecordId),
    }

    impl AssigneeIdRecord {
        pub fn to_entity(&self) -> AssigneeId {
            match self {
                Self::User(rid) => {
                    AssigneeId::User(UserId(rid.clone().key.into_value().into_string().unwrap()))
                }
                Self::Group(rid) => {
                    AssigneeId::Group(GroupId(rid.clone().key.into_value().into_string().unwrap()))
                }
            }
        }
    }

    #[derive(SurrealValue)]
    pub struct AssignedJobRecord {
        pub id: RecordId,
        pub assignee_id: AssigneeIdRecord,
        pub job_id: RecordId,
        pub params_values: BTreeMap<String, Value>,
        pub trigger: JobTriggerRecord,
    }

    impl AssignedJobRecord {
        pub fn to_entity(&self) -> AssignedJob {
            AssignedJob {
                id: Some(AssignedJobId(
                    self.id.clone().key.into_value().into_string().unwrap(),
                )),

                assignee_id: self.assignee_id.to_entity(),
                job_id: JobId(self.job_id.clone().key.into_value().into_string().unwrap()),
                params_values: self.params_values.clone(),

                trigger: self.trigger.to_entity(),
            }
        }
    }
}

pub mod log {
    use super::*;

    #[derive(SurrealValue, Clone)]
    pub enum LogSourceRecord {
        Database,
        Server,
    }

    impl LogSourceRecord {
        pub fn to_entity(&self) -> LogSource {
            match self {
                Self::Database => LogSource::Database,
                Self::Server => LogSource::Server,
            }
        }
    }

    #[derive(SurrealValue)]
    pub struct LogRecord {
        pub id: RecordId,
        pub source: LogSourceRecord,
        pub action: String,
        pub message: Option<String>,
        pub table_affected: Option<String>,
        pub record_id: Option<String>,
        pub timestamp: DateTime<Utc>,
    }

    impl LogRecord {
        pub fn to_entity(&self) -> Log {
            Log {
                id: Some(LogId(
                    self.id.clone().key.into_value().into_string().unwrap(),
                )),
                source: self.source.to_entity(),
                action: self.action.clone(),
                message: self.message.clone(),
                table_affected: self.table_affected.clone(),
                record_id: self.record_id.clone(),
                timestamp: self.timestamp,
            }
        }
    }
}

pub mod user {
    use super::*;

    #[derive(SurrealValue, Clone)]
    pub enum UserTypeRecord {
        Common,
        Admin,
    }

    impl UserTypeRecord {
        pub fn to_entity(&self) -> UserType {
            match self {
                Self::Common => UserType::Common,
                Self::Admin => UserType::Admin,
            }
        }
    }

    #[derive(SurrealValue)]
    pub struct UserRecord {
        pub id: RecordId,
        pub username: String,
        pub active: bool,
        pub password: Option<String>,
        pub email: Option<String>,
        pub user_type: UserTypeRecord,
        pub optional_data: BTreeMap<String, Value>,
        pub domain: Option<RecordId>,
        pub group: RecordId,
        pub weekdays: Vec<WeekdayRecord>,
    }

    impl UserRecord {
        pub fn to_entity(&self) -> User {
            User {
                id: Some(UserId(
                    self.id.clone().key.into_value().into_string().unwrap(),
                )),
                username: self.username.clone(),
                active: self.active,
                password: self.password.clone(),
                email: self.email.clone(),
                user_type: self.user_type.to_entity(),
                optional_data: self.optional_data.clone(),
                domain: self
                    .domain
                    .as_ref()
                    .map(|rid| DomainId(rid.clone().key.into_value().into_string().unwrap())),
                group: GroupId(self.group.clone().key.into_value().into_string().unwrap()),
                weekdays: self.weekdays.iter().map(|w| w.to_entity()).collect(),
            }
        }
    }
}
