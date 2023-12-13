use axum::extract::Json;
use base64::{engine::general_purpose, Engine};
use serde::{Deserialize, Serialize};
use tower_cookies::Cookies;

pub async fn task_1(cookies: Cookies) -> String {
    get_cookie_recipe(cookies)
}

#[derive(Deserialize, Debug)]
struct RecipeAndPantry {
    recipe: Ingredients,
    pantry: Ingredients,
}

#[derive(Serialize, Deserialize, Debug)]
struct Ingredients {
    flour: u64,
    sugar: u64,
    butter: u64,
    #[serde(rename = "baking powder")]
    baking_powder: u64,
    #[serde(rename = "chocolate chips")]
    chocolate_chips: u64,
}

#[derive(Serialize, Debug)]
pub struct CookiesAndPantry {
    cookies: u64,
    pantry: Ingredients,
}

pub async fn task_2(cookies: Cookies) -> Json<CookiesAndPantry> {
    let recipe_and_pantry: RecipeAndPantry =
        serde_json::from_str(&get_cookie_recipe(cookies)).unwrap();
    let recipe = recipe_and_pantry.recipe;
    let pantry = recipe_and_pantry.pantry;

    let cookies = [
        pantry.flour as f64 / recipe.flour as f64,
        pantry.sugar as f64 / recipe.sugar as f64,
        pantry.butter as f64 / recipe.butter as f64,
        pantry.baking_powder as f64 / recipe.baking_powder as f64,
        pantry.chocolate_chips as f64 / recipe.chocolate_chips as f64,
    ]
    .into_iter()
    .map(|value| value.floor() as u64)
    .min()
    .unwrap_or_default();

    let pantry = Ingredients {
        flour: pantry.flour - (cookies * recipe.flour),
        sugar: pantry.sugar - (cookies * recipe.sugar),
        butter: pantry.butter - (cookies * recipe.butter),
        baking_powder: pantry.baking_powder - (cookies * recipe.baking_powder),
        chocolate_chips: pantry.chocolate_chips - (cookies * recipe.chocolate_chips),
    };

    Json(CookiesAndPantry { cookies, pantry })
}

fn get_cookie_recipe(cookies: Cookies) -> String {
    cookies
        .get("recipe")
        .map(|cookie| {
            general_purpose::STANDARD
                .decode(cookie.value())
                .ok()
                .and_then(|decoded| {
                    std::str::from_utf8(&decoded)
                        .map(|decoded| decoded.to_string())
                        .ok()
                })
        })
        .flatten()
        .unwrap()
}
