use axum::extract::Path;
use serde::Deserialize;

const NUMBER_OF_HECTOGRAMS_IN_A_KG: f32 = 10.0;

#[derive(Deserialize)]
struct Pokemon {
    weight: f32,
}

pub async fn task_1(Path(pokedex_number): Path<String>) -> String {
    let pokemon = reqwest::get(format!(
        "https://pokeapi.co/api/v2/pokemon/{pokedex_number}"
    ))
    .await
    .unwrap()
    .json::<Pokemon>()
    .await
    .unwrap();

    (pokemon.weight / NUMBER_OF_HECTOGRAMS_IN_A_KG).to_string()
}
