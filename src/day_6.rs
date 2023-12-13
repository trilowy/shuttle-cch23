use axum::extract::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct ElfCount {
    elf: usize,
    #[serde(rename = "elf on a shelf")]
    elf_on_a_shelf: usize,
    #[serde(rename = "shelf with no elf on it")]
    shelf_with_no_elf_on_it: usize,
}

pub async fn task_1_and_2(body: String) -> Json<ElfCount> {
    Json(count_elves(body))
}

fn count_elves(body: String) -> ElfCount {
    let elf = body.matches("elf").count();

    let elf_on_a_shelf = body
        .match_indices("elf")
        .filter(|(index, _)| {
            body.get(*index..index + "elf on a shelf".len())
                .filter(|&text| text == "elf on a shelf")
                .is_some()
        })
        .count();

    let shelf_with_no_elf_on_it = body
        .match_indices("shelf")
        .filter(|(index, _)| {
            if let Some(begin) = index.checked_sub("elf on a ".len()) {
                body.get(begin..*index)
                    .filter(|&prefix| prefix == "elf on a ")
                    .is_none()
            } else {
                true
            }
        })
        .count();

    ElfCount {
        elf,
        elf_on_a_shelf,
        shelf_with_no_elf_on_it,
    }
}

#[cfg(test)]
mod tests {
    use super::count_elves;

    #[test]
    fn test_count_elves_1() {
        let body = "there is an elf on a shelf on an elf.
there is also another shelf in Belfast."
            .to_string();
        let result = count_elves(body);

        assert_eq!(result.elf, 5);
        assert_eq!(result.elf_on_a_shelf, 1);
        assert_eq!(result.shelf_with_no_elf_on_it, 1);
    }

    #[test]
    fn test_count_elves_2() {
        let body = "In Belfast I heard an elf on a shelf on a shelf on a".to_string();
        let result = count_elves(body);

        assert_eq!(result.elf, 4);
        assert_eq!(result.elf_on_a_shelf, 2);
        assert_eq!(result.shelf_with_no_elf_on_it, 0);
    }
}
