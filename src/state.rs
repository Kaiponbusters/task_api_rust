use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;

use crate::repo::TaskRepository;

// Arc : スレッド間で共有できる参照カウント型スマートポインタ
// axum + tokio環境では、リクエストごとにハンドラが並列実行される。
// AppStateを複数ハンドラで安全に共有することが必要。
// Arcは同じ中身を複数箇所でさせる。
// 最後の参照が消えたときに自動解放される仕組み。

// RwLock : Read/Write Lockの仕組み。
// 非同期環境において.awaitしながらロック待ちをするため。
#[derive(Clone)]
pub struct AppState {
    //Inmemory "DB"
    pub repo: Arc<dyn TaskRepository + Send + Sync>,
    pub next_id: Arc<RwLock<i64>>,
}
impl AppState {
    pub fn new() -> Self {
        Self {
            tasks: Arc::new(RwLock::new(HashMap::new())),
            next_id: Arc::new(RwLock::new(1)),
        }
    }
}
