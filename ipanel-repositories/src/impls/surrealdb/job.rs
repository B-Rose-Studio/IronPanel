use crate::{
    DBClient, ListMethod, Repository, RepositoryError, RepositoryResult,
    interfaces::job::JobRepository,
    surrealdb::dtos::job::{AssignedJobRecord, AssigneeIdRecord, JobRecord, JobTriggerRecord},
};
use ipanel_domain::models::job::{AssignedJob, AssignedJobId, AssigneeId, Job, JobId, JobTrigger};
use surrealdb::{
    Surreal,
    engine::remote::ws::Client,
    types::{RecordId, RecordIdKey},
};

pub struct SurrealJobRepository {
    db: DBClient<Surreal<Client>>,
}

impl SurrealJobRepository {
    pub fn new(db: DBClient<Surreal<Client>>) -> Self {
        Self { db }
    }
}

#[async_trait::async_trait]
impl Repository for SurrealJobRepository {
    type Entity = Job;
    type Id = JobId;

    async fn list(&self, method: ListMethod) -> RepositoryResult<Vec<Job>> {
        let result = match method {
            ListMethod::All => self.db.select("jobs").await,
            ListMethod::Pagined { page, qtd } => {
                let mut response = self
                    .db
                    .query("SELECT * FROM jobs LIMIT $qtd START $page")
                    .bind(("qtd", qtd))
                    .bind(("page", page))
                    .await
                    .map_err(|_| RepositoryError::DataError)?;

                response.take(0)
            }
        };

        match result {
            Ok(records_list) => {
                let records: Vec<JobRecord> = records_list;
                let list: Vec<Job> = records.iter().map(|record| record.to_entity()).collect();
                Ok(list)
            }
            Err(_) => Err(RepositoryError::DatabaseConnection),
        }
    }

    async fn find_by_id(&self, id: JobId) -> RepositoryResult<Job> {
        let record: Option<JobRecord> = self
            .db
            .select(("jobs", id.0.clone()))
            .await
            .map_err(|_| RepositoryError::DatabaseConnection)?;

        match record {
            Some(r) => Ok(r.to_entity()),
            None => Err(RepositoryError::EntityNotFound(id.0.clone())),
        }
    }

    async fn create(&self, entity: Job) -> RepositoryResult<Job> {
        use surrealdb::types::RecordIdKey;

        let record_data = JobRecord {
            id: RecordId::new("jobs", RecordIdKey::rand()),
            name: entity.name,
            description: entity.description,
            content: entity.content,
            params: entity.params,
        };

        let created: Option<JobRecord> = self
            .db
            .create("jobs")
            .content(record_data)
            .await
            .map_err(|e| {
                println!("{e:?}");
                RepositoryError::DataError
            })?;

        Ok(created.unwrap().to_entity())
    }

    async fn update(&self, entity: Job) -> RepositoryResult<Job> {
        let id = entity.id.clone().ok_or(RepositoryError::DataError)?;

        let record_data = JobRecord {
            id: RecordId::new("jobs", id.0.clone()),
            name: entity.name,
            description: entity.description,
            content: entity.content,
            params: entity.params,
        };

        let record: Option<JobRecord> = self
            .db
            .update(("jobs", id.0.clone()))
            .content(record_data)
            .await
            .map_err(|_| RepositoryError::DataError)?;

        match record {
            Some(r) => Ok(r.to_entity()),
            None => Err(RepositoryError::EntityNotFound(id.0.clone())),
        }
    }

    async fn delete(&self, id: JobId) -> RepositoryResult<Job> {
        let record: Option<JobRecord> = self
            .db
            .delete(("jobs", id.0.clone()))
            .await
            .map_err(|_| RepositoryError::DataError)?;

        match record {
            Some(r) => Ok(r.to_entity()),
            None => Err(RepositoryError::EntityNotFound(id.0.clone())),
        }
    }
}

#[async_trait::async_trait]
impl JobRepository for SurrealJobRepository {
    async fn assign_job(&self, assigned_job: AssignedJob) -> RepositoryResult<AssignedJob> {
        let record_data = AssignedJobRecord {
            id: RecordId::new("assigned_job", RecordIdKey::rand()),
            assignee_id: match assigned_job.assignee_id {
                AssigneeId::User(uid) => AssigneeIdRecord::User(RecordId::new("users", uid.0)),
                AssigneeId::Group(gid) => AssigneeIdRecord::Group(RecordId::new("groups", gid.0)),
            },
            job_id: RecordId::new("jobs", assigned_job.job_id.0),
            params_values: assigned_job.params_values,
            trigger: match assigned_job.trigger {
                JobTrigger::Manual => JobTriggerRecord::Manual,
                JobTrigger::OnStartup => JobTriggerRecord::OnStartup,
                JobTrigger::OnLogout => JobTriggerRecord::OnLogout,
                JobTrigger::Interval { seconds } => JobTriggerRecord::Interval { seconds },
                JobTrigger::Cron { expression } => JobTriggerRecord::Cron { expression },
                JobTrigger::SpecificDate { date } => JobTriggerRecord::SpecificDate { date },
            },
        };

        let created: Option<AssignedJobRecord> = self
            .db
            .create("assigned_job")
            .content(record_data)
            .await
            .map_err(|e| {
                println!("{e:?}");
                RepositoryError::DataError
            })?;

        Ok(created.unwrap().to_entity())
    }

    async fn get_assigned_job(&self, id: AssignedJobId) -> RepositoryResult<AssignedJob> {
        let record: Option<AssignedJobRecord> = self
            .db
            .select(("assigned_job", id.0.clone()))
            .await
            .map_err(|_| RepositoryError::DatabaseConnection)?;

        match record {
            Some(r) => Ok(r.to_entity()),
            None => Err(RepositoryError::EntityNotFound(id.0.clone())),
        }
    }

    async fn list_jobs_by_assignee(
        &self,
        assignee_id: AssigneeId,
    ) -> RepositoryResult<Vec<AssignedJob>> {
        let target_assignee = match assignee_id {
            AssigneeId::User(uid) => AssigneeIdRecord::User(RecordId::new("users", uid.0)),
            AssigneeId::Group(gid) => AssigneeIdRecord::Group(RecordId::new("groups", gid.0)),
        };

        let mut response = self
            .db
            .query("SELECT * FROM assigned_job WHERE assignee_id = $target")
            .bind(("target", target_assignee))
            .await
            .map_err(|_| RepositoryError::DataError)?;

        let records: Vec<AssignedJobRecord> =
            response.take(0).map_err(|_| RepositoryError::DataError)?;
        Ok(records.iter().map(|record| record.to_entity()).collect())
    }

    async fn list_assignees_by_job(&self, job_id: JobId) -> RepositoryResult<Vec<AssignedJob>> {
        let target_job = RecordId::new("jobs", job_id.0);

        let mut response = self
            .db
            .query("SELECT * FROM assigned_job WHERE job_id = $target")
            .bind(("target", target_job))
            .await
            .map_err(|_| RepositoryError::DataError)?;

        let records: Vec<AssignedJobRecord> =
            response.take(0).map_err(|_| RepositoryError::DataError)?;
        Ok(records.iter().map(|record| record.to_entity()).collect())
    }

    async fn update_assigned_job(
        &self,
        assigned_job: AssignedJob,
    ) -> RepositoryResult<AssignedJob> {
        let id = assigned_job.id.clone().ok_or(RepositoryError::DataError)?;

        let record_data = AssignedJobRecord {
            id: RecordId::new("assigned_job", id.0.clone()),
            assignee_id: match assigned_job.assignee_id {
                AssigneeId::User(uid) => AssigneeIdRecord::User(RecordId::new("users", uid.0)),
                AssigneeId::Group(gid) => AssigneeIdRecord::Group(RecordId::new("groups", gid.0)),
            },
            job_id: RecordId::new("jobs", assigned_job.job_id.0),
            params_values: assigned_job.params_values,
            trigger: match assigned_job.trigger {
                JobTrigger::Manual => JobTriggerRecord::Manual,
                JobTrigger::OnStartup => JobTriggerRecord::OnStartup,
                JobTrigger::OnLogout => JobTriggerRecord::OnLogout,
                JobTrigger::Interval { seconds } => JobTriggerRecord::Interval { seconds },
                JobTrigger::Cron { expression } => JobTriggerRecord::Cron { expression },
                JobTrigger::SpecificDate { date } => JobTriggerRecord::SpecificDate { date },
            },
        };

        let record: Option<AssignedJobRecord> = self
            .db
            .update(("assigned_job", id.0.clone()))
            .content(record_data)
            .await
            .map_err(|_| RepositoryError::DataError)?;

        match record {
            Some(r) => Ok(r.to_entity()),
            None => Err(RepositoryError::EntityNotFound(id.0.clone())),
        }
    }

    async fn remove_assigned_job(&self, id: AssignedJobId) -> RepositoryResult<AssignedJob> {
        let record: Option<AssignedJobRecord> = self
            .db
            .delete(("assigned_job", id.0.clone()))
            .await
            .map_err(|_| RepositoryError::DataError)?;

        match record {
            Some(r) => Ok(r.to_entity()),
            None => Err(RepositoryError::EntityNotFound(id.0.clone())),
        }
    }
}
