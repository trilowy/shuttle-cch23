use crate::AppState;
use axum::extract::{Path, State};
use std::{sync::Arc, time::Instant};

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
