use base64::{engine::general_purpose, Engine};
use tower_cookies::Cookies;

pub async fn task_1(cookies: Cookies) -> String {
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
