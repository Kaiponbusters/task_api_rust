use crate::handlers::health::health;
use crate::handlers::tasks::create_task;
use crate::handlers::tasks::get_task;
use crate::state::AppState;
use axum::{
    Router,
    routing::{get, post},
};

// with_state()はRouterにアプリ全体で共有したいStateを注入するMethod
// ハンドラでState()というextractorで状態を受け取れる

pub fn route(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/tasks", post(create_task))
        .route("/tasks/{id}", get(get_task))
        .with_state(state)
}
