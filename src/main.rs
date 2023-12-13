use axum::{
    extract::{Json, Path},
    http::StatusCode,
    routing::{get, post},
    Router,
};
use serde::Deserialize;
use shuttle_axum::ShuttleAxum;

async fn day_minus_1_task_1() -> &'static str {
    "Hello, world!"
}

async fn day_minus_1_task_2() -> StatusCode {
    StatusCode::INTERNAL_SERVER_ERROR
}

async fn day_1(Path(numbers): Path<String>) -> String {
    numbers
        .split('/')
        .map(|number| number.parse::<i32>().unwrap())
        .reduce(|acc, e| acc ^ e)
        .unwrap_or_default()
        .pow(3)
        .to_string()
}

#[derive(Deserialize)]
struct Reindeer {
    strength: u32,
}

async fn day_4_task_1(Json(reindeers): Json<Vec<Reindeer>>) -> String {
    reindeers
        .into_iter()
        .map(|reindeer| reindeer.strength)
        .sum::<u32>()
        .to_string()
}

#[shuttle_runtime::main]
async fn main() -> ShuttleAxum {
    let router = Router::new()
        .route("/", get(day_minus_1_task_1))
        .route("/-1/error", get(day_minus_1_task_2))
        .route("/1/*numbers", get(day_1))
        .route("/4/strength", post(day_4_task_1));

    Ok(router.into())
}
