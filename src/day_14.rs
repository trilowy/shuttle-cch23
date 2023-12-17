use askama::Template;
use axum::Json;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Content {
    content: String,
}

#[derive(Template)]
#[template(path = "layout.html")]
pub struct LayoutTemplate {
    content: String,
}

pub async fn task_1(Json(Content { content }): Json<Content>) -> LayoutTemplate {
    LayoutTemplate { content }
}

#[derive(Template)]
#[template(path = "safe_layout.html")]
pub struct SafeLayoutTemplate {
    content: String,
}

pub async fn task_2(Json(Content { content }): Json<Content>) -> SafeLayoutTemplate {
    SafeLayoutTemplate { content }
}
