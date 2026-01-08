use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    routing::{get, post},
};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;

// Deriveは型に対して特定のTrait実装を自動生成させる
// Debug : デバッグ表示の可能化
// Clone : task.clone()を可能にし、値の複製を可能にする
// Serialize : その型をJSONなどにシリアライズできる。
#[derive(Debug, Clone, Serialize)]
struct Task {
    id: i64,
    title: String,
    completed: bool,
    created_at: String, //RFC3339 (UTC)
}

// Deserialize : JSONなどから型へデシリアライズできる
#[derive(Debug, Deserialize)]
struct CreateTaskRequest {
    title: String,
    completed: bool,
}

// Arc : スレッド間で共有できる参照カウント型スマートポインタ
// axum + tokio環境では、リクエストごとにハンドラが並列実行される。
// AppStateを複数ハンドラで安全に共有することが必要。
// Arcは同じ中身を複数箇所でさせる。
// 最後の参照が消えたときに自動解放される仕組み。

// RwLock : Read/Write Lockの仕組み。
// 非同期環境において.awaitしながらロック待ちをするため。
#[derive(Clone)]
struct AppState {
    //Inmemory "DB"
    tasks: Arc<RwLock<HashMap<i64, Task>>>,
    next_id: Arc<RwLock<i64>>,
}

async fn health() -> &'static str {
    "ok"
}

// create_task(
//
// )
async fn create_task(
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

async fn get_task(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<Task>, StatusCode> {
    let guard = state.tasks.read().await;
    match guard.get(&id) {
        Some(task) => Ok(Json(task.clone())),
        None => Err(StatusCode::NOT_FOUND),
    }
}

#[tokio::main]
async fn main() {
    let state = AppState {
        tasks: Arc::new(RwLock::new(HashMap::new())),
        next_id: Arc::new(RwLock::new(1)),
    };

    let app = Router::new()
        .route("/health", get(health))
        .route("/tasks", post(create_task))
        .route("/tasks/{id}", get(get_task))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}
