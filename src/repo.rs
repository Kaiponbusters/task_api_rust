use crate::models::Task;
use crate::state::AppState;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;
use async_trait::async_trait;

#[derive(Debug, thiserror::Error)]
pub enum RepoError {
    #[error("RepoError OK")]
    SimulatedFailure,
}

pub async fn insert_task(
    state: &AppState,
    task: &Task,
    simulate_failure: bool,
) -> Result<(), RepoError> {
    if simulate_failure {
        //TODO:Err(...)
        Err(RepoError::SimulatedFailure)?;
    }
    // TODO:Insert(必要ならclone/借用の意識)
    // taskは借用で受けて、必要箇所だけcloneすることで
    // 所有権境界を明確にする
    state.tasks.write().await.insert(task.id, task.clone());
    Ok(())
}

pub struct InMemoryTaskRepository {
    pub tasks: Arc<RwLock<HashMap<i64, Task>>>,
}

impl InMemoryTaskRepository {
    fn new(&self) -> Self {
        let m = HashMap::new();
        let lock = RwLock::new(m);
        let shared = Arc::new(lock);
        Self { tasks: shared }
    }
}

#[async_trait]
pub trait TaskRepository {
    async fn insert(&self, task: Task) -> Result<(), RepoError>;

    async fn get(&self, id: i64) -> Result<Option<Task>, RepoError>;
}
