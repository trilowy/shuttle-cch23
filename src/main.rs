use axum::{http::StatusCode, routing::get, Router};
use shuttle_axum::ShuttleAxum;

async fn day_minus_1_task_1() -> &'static str {
    "Hello, world!"
}

async fn day_minus_1_task_2() -> StatusCode {
    StatusCode::INTERNAL_SERVER_ERROR
}

#[shuttle_runtime::main]
async fn main() -> ShuttleAxum {
    let router = Router::new()
        .route("/", get(day_minus_1_task_1))
        .route("/-1/error", get(day_minus_1_task_2));

    Ok(router.into())
}
