use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use chrono::Utc;

use crate::models::CreateTaskRequest;
use crate::models::Task;
use crate::state::AppState;

pub async fn create_task(
    State(state): State<AppState>,
    Json(req): Json<CreateTaskRequest>,
) -> (StatusCode, Json<Task>) {
    //NOTE: 今は設計凍結。V/E設計はあとのスプリント.
    let mut id_guard = state.next_id.write().await;
    let id = *id_guard;
    *id_guard += 1;
    drop(id_guard);

    let task = Task {
        id,
        title: req.title,
        completed: req.completed,
        created_at: Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Secs, true),
    };

    state.tasks.write().await.insert(id, task.clone());

    // Return値
    (StatusCode::CREATED, Json(task))
}

pub async fn get_task(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<Task>, StatusCode> {
    let guard = state.tasks.read().await;
    match guard.get(&id) {
        Some(task) => Ok(Json(task.clone())),
        None => Err(StatusCode::NOT_FOUND),
    }
}
