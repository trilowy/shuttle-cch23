use axum::{
    extract::{Json, Path},
    http::StatusCode,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
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

#[derive(Deserialize)]
struct ReindeerContest {
    name: String,
    strength: u32,
    speed: f32,
    height: u32,
    antler_width: u32,
    snow_magic_power: u32,
    favorite_food: String,
    #[serde(rename = "cAnD13s_3ATeN-yesT3rdAy")]
    candies: u32,
}

#[derive(Serialize)]
struct Contest {
    fastest: String,
    tallest: String,
    magician: String,
    consumer: String,
}

async fn day_4_task_2(Json(reindeers): Json<Vec<ReindeerContest>>) -> Json<Contest> {
    let mut fastest: Option<&ReindeerContest> = None;
    let mut tallest: Option<&ReindeerContest> = None;
    let mut magician: Option<&ReindeerContest> = None;
    let mut consumer: Option<&ReindeerContest> = None;

    for reindeer in reindeers.iter() {
        match fastest {
            Some(fastest_reindeer) => {
                if fastest_reindeer.speed < reindeer.speed {
                    fastest = Some(reindeer)
                }
            }
            None => fastest = Some(reindeer),
        }
        match tallest {
            Some(tallest_reindeer) => {
                if tallest_reindeer.height < reindeer.height {
                    tallest = Some(reindeer)
                }
            }
            None => tallest = Some(reindeer),
        }
        match magician {
            Some(magician_reindeer) => {
                if magician_reindeer.snow_magic_power < reindeer.snow_magic_power {
                    magician = Some(reindeer)
                }
            }
            None => magician = Some(reindeer),
        }
        match consumer {
            Some(consumer_reindeer) => {
                if consumer_reindeer.candies < reindeer.candies {
                    consumer = Some(reindeer)
                }
            }
            None => consumer = Some(reindeer),
        }
    }

    Json(Contest {
        fastest: format!(
            "Speeding past the finish line with a strength of {} is {}",
            fastest.unwrap().strength,
            fastest.unwrap().name
        ),
        tallest: format!(
            "{} is standing tall with his {} cm wide antlers",
            tallest.unwrap().name,
            tallest.unwrap().antler_width
        ),
        magician: format!(
            "{} could blast you away with a snow magic power of {}",
            magician.unwrap().name,
            magician.unwrap().snow_magic_power
        ),
        consumer: format!(
            "{} ate lots of candies, but also some {}",
            consumer.unwrap().name,
            consumer.unwrap().favorite_food
        ),
    })
}

#[shuttle_runtime::main]
async fn main() -> ShuttleAxum {
    let router = Router::new()
        .route("/", get(day_minus_1_task_1))
        .route("/-1/error", get(day_minus_1_task_2))
        .route("/1/*numbers", get(day_1))
        .route("/4/strength", post(day_4_task_1))
        .route("/4/contest", post(day_4_task_2));

    Ok(router.into())
}
