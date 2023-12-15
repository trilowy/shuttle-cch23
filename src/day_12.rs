use crate::AppState;
use axum::extract::{Json, Path, State};
use std::{sync::Arc, time::Instant};
use ulid::Ulid;
use uuid::Uuid;

pub async fn task_1_save(Path(string): Path<String>, State(state): State<Arc<AppState>>) {
    state.day_12.lock().unwrap().insert(string, Instant::now());
}

pub async fn task_1_load(Path(string): Path<String>, State(state): State<Arc<AppState>>) -> String {
    state
        .day_12
        .lock()
        .unwrap()
        .get(&string)
        .map(|time| time.elapsed().as_secs())
        .unwrap_or_default()
        .to_string()
}

pub async fn task_2(Json(ulids): Json<Vec<Ulid>>) -> Json<Vec<Uuid>> {
    let uuids = ulids
        .into_iter()
        .rev()
        .map(|ulid| Uuid::from_bytes(ulid.to_bytes()))
        .collect();

    Json(uuids)
}
