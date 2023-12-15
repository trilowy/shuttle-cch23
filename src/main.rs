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
        .route("/4/strength", post(day_4::task_1))
        .route("/4/contest", post(day_4::task_2))
        .route("/6", post(day_6::task_1_and_2))
        .route("/7/decode", get(day_7::task_1))
        .route("/7/bake", get(day_7::task_2_and_3))
        .route("/8/weight/:pokedex_number", get(day_8::task_1))
        .route("/8/drop/:pokedex_number", get(day_8::task_2))
        .route("/11/red_pixels", post(day_11::task_2))
        .route("/12/save/:string", post(day_12::task_1_save))
        .route("/12/load/:string", get(day_12::task_1_load))
        .route("/12/ulids", post(day_12::task_2))
        .route("/12/ulids/:weekday", post(day_12::task_3))
        .route("/13/sql", get(day_13::task_1))
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
