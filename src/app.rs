// use axum::Router;
// use axum::routing::get;
use crate::handlers::health::health;
use crate::handlers::tasks::create_task;
use crate::handlers::tasks::get_task;
use crate::state::AppState;
use axum::{
    Router,
    routing::{get, post},
};

pub fn route(state: AppState) -> Router {
    let app = Router::new()
        .route("/health", get(health))
        .route("/tasks", post(create_task))
        .route("/tasks/{id}", get(get_task))
        .with_state(state);
    app
}
