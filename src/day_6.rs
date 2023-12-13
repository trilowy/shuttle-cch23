use axum::extract::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct ElfCount {
    elf: usize,
}

pub async fn task_1(body: String) -> Json<ElfCount> {
    let elf = body.matches("elf").count();

    Json(ElfCount { elf })
}
