use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use chrono::Utc;

use crate::models::Task;
use crate::state::AppState;
use crate::{error::ApiError, repo};
use crate::{error::validate_title, models::CreateTaskRequest};

pub async fn create_task(
    State(state): State<AppState>,
    Json(req): Json<CreateTaskRequest>,
) -> Result<(StatusCode, Json<Task>), ApiError> {
    validate_title(&req.title)?;
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

    repo::insert_task(&state, &task, false).await?;

    // Return値
    Ok((StatusCode::CREATED, Json(task)))
}

pub async fn get_task(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<Task>, ApiError> {
    let guard = state.tasks.read().await;
    let task = guard.get(&id).cloned().ok_or(ApiError::NotFound)?;
    Ok(Json(task))
}
