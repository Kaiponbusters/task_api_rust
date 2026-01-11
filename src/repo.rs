use crate::state::AppState;
use crate::models::Task;

#[derive(Debug, thiserror::Error)]
pub enum RepoError {
    #[error("RepoError OK")]
    SimulatedFailure,
}

pub async fn insert_task(
    state: &AppState,
    task : &Task,
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