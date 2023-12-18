use axum::extract::Path;

pub async fn task_1_and_2(Path(numbers): Path<String>) -> String {
    numbers
        .split('/')
        .map(|number| number.parse::<i32>().unwrap())
        .reduce(|acc, e| acc ^ e)
        .unwrap_or_default()
        .pow(3)
        .to_string()
}
