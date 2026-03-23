use chrono::{DateTime, Utc};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LogId(pub String);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LogSource {
    Database,
    Server,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Log {
    pub id: Option<LogId>,
    pub source: LogSource,
    pub action: String,
    pub message: Option<String>,
    pub table_affected: Option<String>,
    pub record_id: Option<String>,
    pub timestamp: DateTime<Utc>,
}
