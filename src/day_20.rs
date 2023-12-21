use axum::body::Bytes;
use gix::traverse::commit::Sorting;
use std::str;
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

pub async fn task_2(body: Bytes) -> String {
    let mut tar = Archive::new(body.as_ref());
    let tmp_dir = tempfile::tempdir().unwrap();
    tar.unpack(tmp_dir.path()).unwrap();

    let repo = gix::discover(tmp_dir.path()).unwrap();
    let christmas_branch = repo.find_reference("christmas").unwrap().id();

    let cookie_eater = repo
        .rev_walk([christmas_branch])
        .sorting(Sorting::ByCommitTimeNewestFirst)
        .all()
        .unwrap()
        .flatten()
        .map(|info| info.object().unwrap())
        .find(|commit| {
            commit
                .tree()
                .unwrap()
                .find_entry("santa.txt")
                .map(|entry| {
                    str::from_utf8(entry.object().unwrap().data.as_slice())
                        .unwrap()
                        .contains("COOKIE")
                })
                .unwrap_or(false)
        })
        .map(|commit| {
            format!(
                "{} {}",
                commit.author().unwrap().actor().name,
                commit.id().to_hex().to_string()
            )
        })
        .unwrap();

    cookie_eater
}
