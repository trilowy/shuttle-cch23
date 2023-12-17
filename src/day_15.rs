use axum::{http::StatusCode, Json};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct Input {
    input: String,
}

impl Input {
    fn is_nice(&self) -> bool {
        self.has_at_least_3_vowels()
            && self.has_doubled_letter()
            && self.does_not_contains_substrings()
    }

    fn has_at_least_3_vowels(&self) -> bool {
        let mut count = 0;
        for c in self.input.chars() {
            if "aeiouy".contains(c) {
                count += 1;
            }
            if count == 3 {
                return true;
            }
        }
        false
    }

    fn has_doubled_letter(&self) -> bool {
        let mut previous = None;
        for c in self.input.chars() {
            if Some(c) == previous && c.is_alphabetic() {
                return true;
            }
            previous = Some(c);
        }
        false
    }

    fn does_not_contains_substrings(&self) -> bool {
        !self.input.contains("ab")
            && !self.input.contains("cd")
            && !self.input.contains("pq")
            && !self.input.contains("xy")
    }
}

#[derive(Serialize)]
pub struct Validation {
    result: String,
}

pub async fn task_1(Json(input): Json<Input>) -> (StatusCode, Json<Validation>) {
    let (response_code, result) = if input.is_nice() {
        (StatusCode::OK, "nice")
    } else {
        (StatusCode::BAD_REQUEST, "naughty")
    };

    (
        response_code,
        Json(Validation {
            result: result.to_string(),
        }),
    )
}
