# axum 0.8 の Router と State に関する学び

作成日: 2026/01/09

## 1. main.rs の責務分割

元々 `main.rs` に集中していた処理を、責務ごとに以下のように分割した。

| ファイル | 責務 |
|---------|------|
| `main.rs` | 起動処理（listener, serve） |
| `app.rs` | ルーティング定義 |
| `handlers/*.rs` | リクエスト処理（HTTPの翻訳係） |
| `state.rs` | 共有状態の定義と管理 |
| `models.rs` | データ構造（Task, CreateTaskRequest等） |

### 良い点
- `main.rs` が薄くなり、責務が明確
- handlers が state の内部実装に直接依存しない方向へ整理できる土台ができた

### 改善の余地
- `AppState` の `pub` フィールドを減らし、操作をメソッド化すると依存方向がさらに整う

---

## 2. axum 0.8 の `Router<S>` の意味（重要）

axum 0.8 では `Router<S>` の `S` は **「持っている state」ではなく「不足している（注入待ちの）state」** を表す。

| 型 | 意味 |
|----|------|
| `Router<AppState>` | AppState がまだ注入されていない（不足している）Router |
| `Router<()>` または `Router` | 不足 state が無い Router（完成品） |

### なぜこれが重要か

`axum::serve(listener, router)` に渡せるのは基本的に **`Router<()>`（完成品）** である。

`Router<AppState>` のまま `serve` に渡そうとすると、trait bound が満たされずエラーになる。

---

## 3. `with_state` とは

`with_state` は **Router に共有状態を紐づける（注入する）メソッド**。

```rust
let app = Router::new()
    .route("/tasks", post(create_task))
    .with_state(state);  // ここで state を注入
```

### 動作の流れ
1. 起動時に `let state = AppState::new();`
2. Router に `.with_state(state)` で登録
3. ハンドラは `State<AppState>` extractor で受け取る

### 重要なポイント
- `.with_state(state)` を呼ぶと、不足 state が埋まり `Router<()>` 側に寄る
- グローバル変数ではなく、明示的に Router に紐づけるので安全
- 毎リクエスト state を作り直すわけではない（共有される）

---

## 4. 設計パターン：`with_state` をどこで呼ぶか

### パターンA：route関数内で state を注入（完成品を返す）

```rust
// app.rs
pub fn route(state: AppState) -> Router {
    Router::new()
        .route("/tasks", post(create_task))
        .with_state(state)
}

// main.rs
let app = app::route(state);
axum::serve(listener, app).await.unwrap();
```

- `route` の責務：ルーティング定義 + state 注入
- 戻り値：`Router`（= `Router<()>`、完成品）

### パターンB：route関数はルーティング定義のみ（main で state 注入）

```rust
// app.rs
pub fn route(router: Router<AppState>) -> Router<AppState> {
    router
        .route("/tasks", post(create_task))
}

// main.rs
let router = axum::Router::<AppState>::new();
let app = app::route(router).with_state(state);
axum::serve(listener, app).await.unwrap();
```

- `route` の責務：ルーティング定義のみ
- 戻り値：`Router<AppState>`（state 不足のまま）
- `main.rs` で `.with_state(state)` して完成させる

### どちらを選ぶか
- 小規模・シンプルなら **パターンA** が楽
- 複数ルータの合成（nest/merge）や拡張性重視なら **パターンB** が自然

---

## 5. `into_make_service()` について

### 何をするメソッドか

「Request を処理する Service」を「IncomingStream（接続）を処理する Service」に変換する。

- 入力：`Service<Request<Body>>`（HTTPリクエストを処理できる）
- 出力：`Service<IncomingStream<...>>`（接続ごとにServiceを用意できる）

### いつ必要か

| 状況 | `into_make_service()` |
|------|----------------------|
| 普通に起動するだけ（`Router<()>` を `serve` に渡す） | 不要 |
| ルータ全体に tower ミドルウェアを巻く | 必要になることがある |
| 接続情報（クライアントIP等）を `ConnectInfo` で取りたい | `into_make_service_with_connect_info` を使う |

### 注意点

`into_make_service()` には2種類ある：
- `Router::into_make_service`（Router 固有メソッド、`Router<()>` にのみ存在）
- `ServiceExt::into_make_service`（拡張 trait のメソッド、`use axum::ServiceExt;` が必要）

---

## 6. 「トレイト境界が満たされていない」エラーの読み方

### エラーの意味

> その場所で必要とされている能力（trait）を、今の型が持っていない（実装していない）状態

### エラーメッセージの読み方

```
the trait bound 'X: Trait<Y>' is not satisfied
```

- `X`：渡した型（例：`Router<AppState>`）
- `Trait<Y>`：要求されている能力（例：`Service<IncomingStream<...>>`）

### 今回のケースでの原因

`axum::serve` は `Service<IncomingStream<...>>` を要求するが、`Router<AppState>` はその trait を実装していなかった。

解決策：`.with_state(state)` で `Router<()>` にしてから `serve` に渡す。

---

## 7. inherent method と trait method の違い

Rust で「メソッドが存在しません」と言われたときの切り分け。

| 種類 | 説明 | 呼び方 |
|------|------|--------|
| inherent method | 型に直接定義されている | `value.method()` |
| trait method | trait に定義されている | `use Trait;` してから `value.method()` |

同名メソッドが両方に存在する場合、どちらが呼ばれるかは状況依存。

### 例：`into_make_service`
- `Router::into_make_service`（inherent、`Router<()>` にのみ存在）
- `ServiceExt::into_make_service`（trait method、`use axum::ServiceExt;` が必要）

「メソッドが無い」と言われたら、「inherent か trait か？ trait なら import してるか？」を確認する。

---

## 8. 学びのまとめ

1. axum 0.8 の `Router<S>` は「S が不足している」という意味
2. `serve` には `Router<()>`（完成品）を渡す
3. `.with_state(state)` で不足 state を埋める
4. `into_make_service()` は普通の起動には不要
5. エラーは「型が何を実装しているか」「何が要求されているか」で読む
6. trait method は `use` しないと見えない
