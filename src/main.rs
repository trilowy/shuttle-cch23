use axum::{extract::Path, http::StatusCode, routing::get, Router};
use shuttle_axum::ShuttleAxum;

async fn day_minus_1_task_1() -> &'static str {
    "Hello, world!"
}

async fn day_minus_1_task_2() -> StatusCode {
    StatusCode::INTERNAL_SERVER_ERROR
}

async fn day_1(Path((num_1, num_2)): Path<(u32, u32)>) -> String {
    (num_1 ^ num_2).pow(3).to_string()
}

#[shuttle_runtime::main]
async fn main() -> ShuttleAxum {
    let router = Router::new()
        .route("/", get(day_minus_1_task_1))
        .route("/-1/error", get(day_minus_1_task_2))
        .route("/1/:num_1/:num_2", get(day_1));

    Ok(router.into())
}
