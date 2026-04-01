use crate::{Repository, RepositoryResult};
use ipanel_domain::models::job::{AssignedJob, AssignedJobId, AssigneeId, Job, JobId};

#[async_trait::async_trait]
pub trait JobRepository: Send + Sync + Repository<Entity = Job, Id = JobId> {
    async fn assign_job(&self, assigned_job: AssignedJob) -> RepositoryResult<AssignedJob>;

    async fn get_assigned_job(&self, id: AssignedJobId) -> RepositoryResult<AssignedJob>;

    async fn list_jobs_by_assignee(
        &self,
        assignee_id: AssigneeId,
    ) -> RepositoryResult<Vec<AssignedJob>>;

    async fn list_assignees_by_job(&self, job_id: JobId) -> RepositoryResult<Vec<AssignedJob>>;

    async fn update_assigned_job(&self, assigned_job: AssignedJob)
    -> RepositoryResult<AssignedJob>;

    async fn remove_assigned_job(&self, id: AssignedJobId) -> RepositoryResult<AssignedJob>;
}
