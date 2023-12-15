use crate::AppState;
use axum::extract::{Json, Path, State};
use chrono::{DateTime, Datelike, Utc, Weekday};
use serde::Serialize;
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
        .map(|ulid| Uuid::from(ulid))
        .collect();

    Json(uuids)
}

#[derive(Serialize)]
pub struct Stats {
    #[serde(rename = "christmas eve")]
    christmas_eve: u32,
    weekday: u32,
    #[serde(rename = "in the future")]
    in_the_future: u32,
    #[serde(rename = "LSB is 1")]
    lsb_is_1: u32,
}

pub async fn task_3(Path(weekday): Path<u8>, Json(ulids): Json<Vec<Ulid>>) -> Json<Stats> {
    let weekday = Weekday::try_from(weekday).unwrap();
    let now = Utc::now();

    let stats = ulids
        .iter()
        .map(|ulid| {
            (
                DateTime::<Utc>::from(ulid.datetime()),
                ulid.to_bytes().last().unwrap() & 1,
            )
        })
        .fold(
            Stats {
                christmas_eve: 0,
                weekday: 0,
                in_the_future: 0,
                lsb_is_1: 0,
            },
            |mut acc, (date, lsb)| {
                if date.day() == 24 && date.month() == 12 {
                    acc.christmas_eve += 1;
                }
                if date.weekday() == weekday {
                    acc.weekday += 1;
                }
                if date > now {
                    acc.in_the_future += 1;
                }
                if lsb == 1 {
                    acc.lsb_is_1 += 1;
                }

                acc
            },
        );

    Json(stats)
}
