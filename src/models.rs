use serde::{Deserialize, Serialize};

// Deriveは型に対して特定のTrait実装を自動生成させる
// Debug : デバッグ表示の可能化
// Clone : task.clone()を可能にし、値の複製を可能にする
// Serialize : その型をJSONなどにシリアライズできる。
#[derive(Debug, Clone, Serialize)]
pub struct Task {
    pub id: i64,
    pub title: String,
    pub completed: bool,
    pub created_at: String, //RFC3339 (UTC)
}

// Deserialize : JSONなどから型へデシリアライズできる
#[derive(Debug, Deserialize)]
pub struct CreateTaskRequest {
    pub title: String,
    pub completed: bool,
}
