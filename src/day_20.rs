use axum::body::Bytes;
use tar::Archive;

pub async fn task_1_number_of_files(body: Bytes) -> String {
    let mut tar = Archive::new(body.as_ref());

    tar.entries().unwrap().count().to_string()
}

pub async fn task_1_file_size(body: Bytes) -> String {
    let mut tar = Archive::new(body.as_ref());

    tar.entries()
        .unwrap()
        .map(|file| file.unwrap().header().size().unwrap())
        .sum::<u64>()
        .to_string()
}
