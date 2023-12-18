use axum::extract::Json;
use base64::{engine::general_purpose, Engine};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tower_cookies::Cookies;

pub async fn task_1(cookies: Cookies) -> String {
    get_cookie_recipe(cookies)
}

#[derive(Deserialize, Debug)]
struct RecipeAndPantry {
    recipe: HashMap<String, u64>,
    pantry: HashMap<String, u64>,
}

#[derive(Serialize, Debug)]
pub struct CookiesAndPantry {
    cookies: u64,
    pantry: HashMap<String, u64>,
}

pub async fn task_2_and_3(cookies: Cookies) -> Json<CookiesAndPantry> {
    let recipe_and_pantry: RecipeAndPantry =
        serde_json::from_str(&get_cookie_recipe(cookies)).unwrap();
    let recipe: HashMap<String, u64> = recipe_and_pantry
        .recipe
        .into_iter()
        .filter(|(_, quantity)| *quantity != 0)
        .collect();
    let pantry = recipe_and_pantry.pantry;

    let cookies = recipe
        .iter()
        .map(|(ingredient, quantity)| {
            pantry
                .get(ingredient)
                .map(|&value| value)
                .unwrap_or_default() as f64
                / *quantity as f64
        })
        .map(|value| value.floor() as u64)
        .min()
        .unwrap_or_default();

    let pantry = pantry
        .into_iter()
        .map(|(ingredient, quantity)| {
            let quantity = quantity
                - (cookies
                    * recipe
                        .get(&ingredient)
                        .map(|&value| value)
                        .unwrap_or_default());

            (ingredient, quantity)
        })
        .collect();

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
