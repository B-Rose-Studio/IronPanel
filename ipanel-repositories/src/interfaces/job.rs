use crate::Repository;
use ipanel_domain::models::job::{Job, JobId};

#[async_trait::async_trait]
pub trait JobRepository: Repository<Entity = Job, Id = JobId> {}
