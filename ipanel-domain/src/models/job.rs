use super::{group::GroupId, user::UserId};
use chrono::{DateTime, Utc};
use serde_json::Value;
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JobId(pub String);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AssigneeId {
    User(UserId),
    Group(GroupId),
}

#[derive(Debug, Clone, PartialEq)]
pub enum JobTrigger {
    Manual,
    OnStartup,
    OnLogout,
    Interval { seconds: u64 },
    Cron { expression: String },
    SpecificDate { date: DateTime<Utc> },
}

#[derive(Debug, Clone, PartialEq)]
pub struct Job {
    pub id: Option<JobId>,
    pub name: String,
    pub description: Option<String>,
    pub content: String,
    pub params: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AssignedJob {
    pub id: Option<String>,
    pub assignee_id: AssigneeId,
    pub job_id: JobId,
    pub params_values: BTreeMap<String, Value>,
    pub trigger: JobTrigger,
}
