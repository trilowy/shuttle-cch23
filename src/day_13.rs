use crate::AppState;
use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Row};
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

pub async fn task_2_reset(State(state): State<Arc<AppState>>) {
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
pub struct Order {
    id: i32,
    region_id: i32,
    gift_name: String,
    quantity: i32,
}

pub async fn task_2_orders(State(state): State<Arc<AppState>>, Json(orders): Json<Vec<Order>>) {
    for order in orders {
        sqlx::query(
            r#"
                INSERT INTO orders (id, region_id, gift_name, quantity)
                VALUES ($1, $2, $3, $4)
            "#,
        )
        .bind(order.id)
        .bind(order.region_id)
        .bind(order.gift_name)
        .bind(order.quantity)
        .fetch_optional(&state.pool)
        .await
        .unwrap();
    }
}

#[derive(Serialize, FromRow, Default)]
pub struct Total {
    total: i64,
}

pub async fn task_2_total(State(state): State<Arc<AppState>>) -> Json<Total> {
    let total = sqlx::query_as::<_, Total>(
        r#"
            SELECT sum(quantity) AS total
            FROM orders
        "#,
    )
    .fetch_optional(&state.pool)
    .await
    .ok()
    .flatten()
    .unwrap_or_default();

    Json(total)
}

#[derive(Serialize, FromRow, Default)]
pub struct Popular {
    popular: Option<String>,
}

pub async fn task_3(State(state): State<Arc<AppState>>) -> Json<Popular> {
    let popular = sqlx::query_as::<_, Popular>(
        r#"
            SELECT gift_name AS popular,
                sum(quantity) AS total_quantity
            FROM orders
            GROUP BY gift_name
            ORDER BY total_quantity DESC
            LIMIT 1
        "#,
    )
    .fetch_optional(&state.pool)
    .await
    .ok()
    .flatten()
    .unwrap_or_default();

    Json(popular)
}
