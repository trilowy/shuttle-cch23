use axum::extract::Json;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct Reindeer {
    strength: u32,
}

pub async fn task_1(Json(reindeers): Json<Vec<Reindeer>>) -> String {
    reindeers
        .into_iter()
        .map(|reindeer| reindeer.strength)
        .sum::<u32>()
        .to_string()
}

#[derive(Deserialize)]
pub struct ReindeerContest {
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
pub struct Contest {
    fastest: String,
    tallest: String,
    magician: String,
    consumer: String,
}

pub async fn task_2(Json(reindeers): Json<Vec<ReindeerContest>>) -> Json<Contest> {
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
