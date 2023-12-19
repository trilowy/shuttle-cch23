use axum::{
    extract::{Path, State},
    Json,
};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};

pub async fn task_1_reset(State(pool): State<PgPool>) {
    sqlx::query(
        r#"
            DROP TABLE IF EXISTS regions
        "#,
    )
    .fetch_optional(&pool)
    .await
    .unwrap();

    sqlx::query(
        r#"
            DROP TABLE IF EXISTS orders
        "#,
    )
    .fetch_optional(&pool)
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
    .fetch_optional(&pool)
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
    .fetch_optional(&pool)
    .await
    .unwrap();
}

#[derive(Deserialize)]
pub struct Region {
    id: i32,
    name: String,
}

pub async fn task_1_regions(State(pool): State<PgPool>, Json(regions): Json<Vec<Region>>) {
    for region in regions {
        sqlx::query(
            r#"
                INSERT INTO regions (id, name)
                VALUES ($1, $2)
            "#,
        )
        .bind(region.id)
        .bind(region.name)
        .fetch_optional(&pool)
        .await
        .unwrap();
    }
}

#[derive(Serialize, FromRow, Default)]
pub struct Total {
    region: String,
    total: i64,
}

pub async fn task_1_total(State(pool): State<PgPool>) -> Json<Vec<Total>> {
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
    .fetch_all(&pool)
    .await
    .unwrap();

    Json(totals)
}

#[derive(Serialize, FromRow, Default)]
pub struct TopGift {
    region: String,
    top_gifts: Vec<String>,
}

pub async fn task_2(Path(limit): Path<i32>, State(pool): State<PgPool>) -> Json<Vec<TopGift>> {
    let top_gifts = sqlx::query_as::<_, TopGift>(
        r#"
            SELECT r.name AS region,
                array_remove(array_agg(o.gift_name), NULL) AS top_gifts
            FROM regions r
            LEFT JOIN LATERAL (
                SELECT o.gift_name,
                    sum(o.quantity) AS total_quantity
                FROM orders o
                WHERE o.region_id = r.id
                GROUP BY o.gift_name
                ORDER BY total_quantity DESC,
                    o.gift_name ASC
                LIMIT $1
                ) o ON TRUE
            GROUP BY r.name
            ORDER BY r.name ASC
        "#,
    )
    .bind(limit)
    .fetch_all(&pool)
    .await
    .unwrap();

    Json(top_gifts)
}
