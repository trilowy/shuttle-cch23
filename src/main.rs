use axum::{
    routing::{get, post},
    Router,
};
use shuttle_axum::ShuttleAxum;
use sqlx::PgPool;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    time::Instant,
};
use tower_cookies::CookieManagerLayer;
use tower_http::services::ServeDir;

mod day_1;
mod day_11;
mod day_12;
mod day_13;
mod day_14;
mod day_15;
mod day_4;
mod day_6;
mod day_7;
mod day_8;
mod day_minus_1;

#[shuttle_runtime::main]
async fn main(#[shuttle_shared_db::Postgres] pool: PgPool) -> ShuttleAxum {
    let shared_state = Arc::new(AppState::new(pool));

    let router = Router::new()
        .route("/", get(day_minus_1::task_1))
        .route("/-1/error", get(day_minus_1::task_2))
        .route("/1/*numbers", get(day_1::task_1_and_2))
        .nest(
            "/4",
            Router::new()
                .route("/strength", post(day_4::task_1))
                .route("/contest", post(day_4::task_2)),
        )
        .route("/6", post(day_6::task_1_and_2))
        .nest(
            "/7",
            Router::new()
                .route("/decode", get(day_7::task_1))
                .route("/bake", get(day_7::task_2_and_3)),
        )
        .nest(
            "/8",
            Router::new()
                .route("/weight/:pokedex_number", get(day_8::task_1))
                .route("/drop/:pokedex_number", get(day_8::task_2)),
        )
        .route("/11/red_pixels", post(day_11::task_2))
        .nest(
            "/12",
            Router::new()
                .route("/save/:string", post(day_12::task_1_save))
                .route("/load/:string", get(day_12::task_1_load))
                .route("/ulids", post(day_12::task_2))
                .route("/ulids/:weekday", post(day_12::task_3)),
        )
        .nest(
            "/13",
            Router::new()
                .route("/sql", get(day_13::task_1))
                .route("/reset", post(day_13::task_2_reset))
                .route("/orders", post(day_13::task_2_orders))
                .route("/orders/total", get(day_13::task_2_total))
                .route("/orders/popular", get(day_13::task_3)),
        )
        .nest(
            "/14",
            Router::new()
                .route("/unsafe", post(day_14::task_1))
                .route("/safe", post(day_14::task_2)),
        )
        .route("/15/nice", post(day_15::task_1))
        .nest_service("/11/assets", ServeDir::new("assets"))
        .layer(CookieManagerLayer::new())
        .with_state(shared_state);

    Ok(router.into())
}

struct AppState {
    day_12: Mutex<HashMap<String, Instant>>,
    pool: PgPool,
}

impl AppState {
    fn new(pool: PgPool) -> Self {
        Self {
            day_12: Mutex::new(HashMap::new()),
            pool,
        }
    }
}
