use axum::{extract::Path, http::StatusCode, routing::get, Router};
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

#[shuttle_runtime::main]
async fn main() -> ShuttleAxum {
    let router = Router::new()
        .route("/", get(day_minus_1_task_1))
        .route("/-1/error", get(day_minus_1_task_2))
        .route("/1/*numbers", get(day_1));

    Ok(router.into())
}
