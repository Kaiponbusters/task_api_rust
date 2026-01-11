use crate::state::AppState;
mod app;
mod error;
mod handlers;
mod models;
mod repo;
mod state;

#[tokio::main]
async fn main() {
    let state = AppState::new();
    let app = app::route(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    // make_service()はIncomingStreamを受け取れるServiceである必要がある。
    // appはHTTPリクエストを受け取れるがIncomingStreamは受け取れない。
    axum::serve(listener, app).await.unwrap();
}
