use crate::AppState;
use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use std::sync::Arc;

pub async fn task_1_reset(State(state): State<Arc<AppState>>) {
    sqlx::query(
        r#"
            DROP TABLE IF EXISTS regions
        "#,
    )
    .fetch_optional(&state.pool)
    .await
    .unwrap();

    sqlx::query(
        r#"
            DROP TABLE IF EXISTS orders
        "#,
    )
    .fetch_optional(&state.pool)
    .await
    .unwrap();

    sqlx::query(
        r#"
            CREATE TABLE regions (
                id INT PRIMARY KEY,
                name VARCHAR(50)
            )
        "#,
    )
    .fetch_optional(&state.pool)
    .await
    .unwrap();

    sqlx::query(
        r#"
            CREATE TABLE orders (
                id INT PRIMARY KEY,
                region_id INT,
                gift_name VARCHAR(50),
                quantity INT
            )
        "#,
    )
    .fetch_optional(&state.pool)
    .await
    .unwrap();
}

#[derive(Deserialize)]
pub struct Region {
    id: i32,
    name: String,
}

pub async fn task_1_regions(State(state): State<Arc<AppState>>, Json(regions): Json<Vec<Region>>) {
    for region in regions {
        sqlx::query(
            r#"
                INSERT INTO regions (id, name)
                VALUES ($1, $2)
            "#,
        )
        .bind(region.id)
        .bind(region.name)
        .fetch_optional(&state.pool)
        .await
        .unwrap();
    }
}

#[derive(Serialize, FromRow, Default)]
pub struct Total {
    region: String,
    total: i64,
}

pub async fn task_1_total(State(state): State<Arc<AppState>>) -> Json<Vec<Total>> {
    let totals = sqlx::query_as::<_, Total>(
        r#"
            SELECT r.name AS region,
                sum(o.quantity) AS total
            FROM orders o
                JOIN regions r
                    ON o.region_id = r.id
            GROUP BY r.name
            ORDER BY r.name ASC
        "#,
    )
    .fetch_all(&state.pool)
    .await
    .unwrap();

    Json(totals)
}
