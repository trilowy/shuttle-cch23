use axum::{
    extract::{
        ws::{Message, WebSocket},
        FromRef,
    },
    routing::{get, post},
    Router,
};
use futures::stream::SplitSink;
use shuttle_axum::ShuttleAxum;
use sqlx::PgPool;
use std::{collections::HashMap, sync::Arc, time::Instant};
use tokio::sync::RwLock;
use tower_cookies::CookieManagerLayer;
use tower_http::services::ServeDir;

mod day_00;
mod day_01;
mod day_04;
mod day_06;
mod day_07;
mod day_08;
mod day_11;
mod day_12;
mod day_13;
mod day_14;
mod day_15;
mod day_18;
mod day_19;

#[shuttle_runtime::main]
async fn main(#[shuttle_shared_db::Postgres] pool: PgPool) -> ShuttleAxum {
    let shared_state = AppState::new(pool);

    let router = Router::new()
        .route("/", get(day_00::task_1))
        .route("/-1/error", get(day_00::task_2))
        .route("/1/*numbers", get(day_01::task_1_and_2))
        .nest(
            "/4",
            Router::new()
                .route("/strength", post(day_04::task_1))
                .route("/contest", post(day_04::task_2)),
        )
        .route("/6", post(day_06::task_1_and_2))
        .nest(
            "/7",
            Router::new()
                .route("/decode", get(day_07::task_1))
                .route("/bake", get(day_07::task_2_and_3)),
        )
        .nest(
            "/8",
            Router::new()
                .route("/weight/:pokedex_number", get(day_08::task_1))
                .route("/drop/:pokedex_number", get(day_08::task_2)),
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
        .nest(
            "/15",
            Router::new()
                .route("/nice", post(day_15::task_1))
                .route("/game", post(day_15::task_2)),
        )
        .nest(
            "/18",
            Router::new()
                .route("/reset", post(day_18::task_1_reset))
                .route("/orders", post(day_13::task_2_orders))
                .route("/regions", post(day_18::task_1_regions))
                .route("/regions/total", get(day_18::task_1_total))
                .route("/regions/top_list/:number", get(day_18::task_2)),
        )
        .nest(
            "/19",
            Router::new()
                .route("/ws/ping", get(day_19::task_1))
                .route("/reset", post(day_19::task_2_reset))
                .route("/views", get(day_19::task_2_views))
                .route("/ws/room/:room_number/user/:user", get(day_19::task_2_room)),
        )
        .nest_service("/11/assets", ServeDir::new("assets"))
        .layer(CookieManagerLayer::new())
        .with_state(shared_state);

    Ok(router.into())
}

type Day12Database = Arc<RwLock<HashMap<String, Instant>>>;

type Day19Views = Arc<RwLock<u32>>;

type Day19Rooms = Arc<RwLock<HashMap<u32, Vec<SplitSink<WebSocket, Message>>>>>;

#[derive(Clone)]
struct AppState {
    day_12_database: Day12Database,
    pool: PgPool,
    day_19_views: Day19Views,
    day_19_rooms: Day19Rooms,
}

impl AppState {
    fn new(pool: PgPool) -> Self {
        Self {
            day_12_database: Arc::new(RwLock::new(HashMap::new())),
            pool,
            day_19_views: Arc::new(RwLock::new(0)),
            day_19_rooms: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

impl FromRef<AppState> for Day12Database {
    fn from_ref(app_state: &AppState) -> Day12Database {
        app_state.day_12_database.clone()
    }
}

impl FromRef<AppState> for PgPool {
    fn from_ref(app_state: &AppState) -> PgPool {
        app_state.pool.clone()
    }
}

impl FromRef<AppState> for Day19Views {
    fn from_ref(app_state: &AppState) -> Day19Views {
        app_state.day_19_views.clone()
    }
}

impl FromRef<AppState> for Day19Rooms {
    fn from_ref(app_state: &AppState) -> Day19Rooms {
        app_state.day_19_rooms.clone()
    }
}
