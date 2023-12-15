use crate::AppState;
use axum::extract::State;
use sqlx::Row;
use std::sync::Arc;

pub async fn task_1(State(state): State<Arc<AppState>>) -> String {
    sqlx::query(
        r#"
            SELECT 20231213
        "#,
    )
    .fetch_optional(&state.pool)
    .await
    .unwrap()
    .unwrap()
    .get::<i32, _>(0)
    .to_string()
}
