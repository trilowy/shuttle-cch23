use axum::{
    routing::{get, post},
    Router,
};
use shuttle_axum::ShuttleAxum;
use tower_cookies::CookieManagerLayer;
use tower_http::services::ServeDir;

mod day_1;
mod day_4;
mod day_6;
mod day_7;
mod day_8;
mod day_minus_1;

#[shuttle_runtime::main]
async fn main() -> ShuttleAxum {
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
        .nest_service("/11/assets", ServeDir::new("assets"))
        .layer(CookieManagerLayer::new());

    Ok(router.into())
}
