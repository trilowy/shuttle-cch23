use axum::extract::Path;
use serde::Deserialize;

const NUMBER_OF_HECTOGRAMS_IN_A_KG: f32 = 10.0;
const GRAVITATIONAL_ACCELERATION: f32 = 9.825; // in m/s²
const CHIMNEY_HEIGHT_IN_METERS: f32 = 10.0;

#[derive(Deserialize)]
struct Pokemon {
    weight: f32,
}

pub async fn task_1(Path(pokedex_number): Path<String>) -> String {
    get_pokemon_weight_in_kg(pokedex_number).await.to_string()
}

pub async fn task_2(Path(pokedex_number): Path<String>) -> String {
    let weight = get_pokemon_weight_in_kg(pokedex_number).await;
    let speed = (2.0 * GRAVITATIONAL_ACCELERATION * CHIMNEY_HEIGHT_IN_METERS).sqrt();

    (weight * speed).to_string()
}

async fn get_pokemon_weight_in_kg(pokedex_number: String) -> f32 {
    let pokemon = reqwest::get(format!(
        "https://pokeapi.co/api/v2/pokemon/{pokedex_number}"
    ))
    .await
    .unwrap()
    .json::<Pokemon>()
    .await
    .unwrap();

    pokemon.weight / NUMBER_OF_HECTOGRAMS_IN_A_KG
}
