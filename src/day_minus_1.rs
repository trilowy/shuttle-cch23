use axum::http::StatusCode;

pub async fn task_1() -> &'static str {
    "Hello, world!"
}

pub async fn task_2() -> StatusCode {
    StatusCode::INTERNAL_SERVER_ERROR
}
