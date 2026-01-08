use axum;

use crate::state::AppState;

mod app;
mod handlers;
mod models;
mod state;

#[tokio::main]
async fn main() {
    let state = AppState::new();
    let app = app::route(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}
