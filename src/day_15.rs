use axum::{http::StatusCode, Json};
use serde::{Deserialize, Serialize};

const NICE: &str = "nice";
const NAUGHTY: &str = "naughty";

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
        (StatusCode::OK, NICE)
    } else {
        (StatusCode::BAD_REQUEST, NAUGHTY)
    };

    (
        response_code,
        Json(Validation {
            result: result.to_string(),
        }),
    )
}

#[derive(Deserialize)]
pub struct GameInput {
    input: String,
}

impl GameInput {
    /// Rule 1: must be at least 8 characters long
    fn break_rule_1(&self) -> bool {
        self.input.len() < 8
    }

    /// Rule 2: must contain uppercase letters, lowercase letters, and digits
    fn break_rule_2(&self) -> bool {
        let mut has_uppercase = false;
        let mut has_lowercase = false;
        let mut has_digit = false;

        for c in self.input.chars() {
            if c.is_numeric() {
                has_digit = true;
            } else if c.is_uppercase() {
                has_uppercase = true;
            } else if c.is_lowercase() {
                has_lowercase = true;
            }

            if has_uppercase && has_lowercase && has_digit {
                return false;
            }
        }

        true
    }

    /// Rule 3: must contain at least 5 digits
    fn break_rule_3(&self) -> bool {
        self.input.chars().filter(|c| c.is_numeric()).count() < 5
    }

    /// Rule 4: all integers (sequences of consecutive digits) in the string must add up to 2023
    fn break_rule_4(&self) -> bool {
        self.input
            .chars()
            .map(|c| if c.is_numeric() { c } else { ' ' })
            .collect::<String>()
            .split_whitespace()
            .map(|int| int.parse::<u32>().unwrap())
            .sum::<u32>()
            != 2023
    }

    /// Rule 5: must contain the letters j, o, and y in that order and in no other order
    fn break_rule_5(&self) -> bool {
        let mut chars = self.input.chars();
        let mut last_last_c = match chars.next() {
            Some(c) => c,
            None => return true,
        };
        let mut last_c = match chars.next() {
            Some(c) => c,
            None => return true,
        };
        let mut found_joy = false;
        for c in self.input.chars() {
            if c.is_alphabetic() {
                match (last_last_c, last_c, c) {
                    (_, 'j', 'o') => (),
                    (_, _, 'o') => return true,
                    ('j', 'o', 'y') => found_joy = true,
                    (_, _, 'y') | ('o', 'y', 'j') | ('y', _, 'j') => return true,
                    (_, _, 'j') => (),
                    (_, 'j', _) => return true,
                    _ => (),
                }
                last_last_c = last_c;
                last_c = c;
            }
        }
        !found_joy
    }

    /// Rule 6: must contain a letter that repeats with exactly one other letter between them (like xyx)
    fn break_rule_6(&self) -> bool {
        let mut chars = self.input.chars();
        let mut last_last_c = match chars.next() {
            Some(c) => c,
            None => return true,
        };
        let mut last_c = match chars.next() {
            Some(c) => c,
            None => return true,
        };
        for c in chars {
            if c == last_last_c && c.is_alphabetic() && last_c.is_alphabetic() {
                return false;
            }
            last_last_c = last_c;
            last_c = c;
        }
        true
    }

    /// Rule 7: must contain at least one unicode character in the range [U+2980, U+2BFF]
    fn break_rule_7(&self) -> bool {
        for c in self.input.chars() {
            match c {
                '\u{2980}'..='\u{2BFF}' => return false,
                _ => (),
            }
        }
        true
    }

    /// Rule 8: must contain at least one emoji
    fn break_rule_8(&self) -> bool {
        for c in self.input.chars() {
            match emojis::get(c.to_string().as_str()) {
                Some(_) => return false,
                None => (),
            }
        }
        true
    }

    /// Rule 9: the hexadecimal representation of the sha256 hash of the string must end with an a
    fn break_rule_9(&self) -> bool {
        !sha256::digest(&self.input).ends_with('a')
    }
}

#[derive(Serialize)]
pub struct GameValidation {
    result: String,
    reason: String,
}

pub async fn task_2(Json(input): Json<GameInput>) -> (StatusCode, Json<GameValidation>) {
    let (response_code, result, reason) = if input.break_rule_1() {
        (StatusCode::BAD_REQUEST, NAUGHTY, "8 chars")
    } else if input.break_rule_2() {
        (StatusCode::BAD_REQUEST, NAUGHTY, "more types of chars")
    } else if input.break_rule_3() {
        (StatusCode::BAD_REQUEST, NAUGHTY, "55555")
    } else if input.break_rule_4() {
        (StatusCode::BAD_REQUEST, NAUGHTY, "math is hard")
    } else if input.break_rule_5() {
        (StatusCode::NOT_ACCEPTABLE, NAUGHTY, "not joyful enough")
    } else if input.break_rule_6() {
        (
            StatusCode::UNAVAILABLE_FOR_LEGAL_REASONS,
            NAUGHTY,
            "illegal: no sandwich",
        )
    } else if input.break_rule_7() {
        (StatusCode::RANGE_NOT_SATISFIABLE, NAUGHTY, "outranged")
    } else if input.break_rule_8() {
        (StatusCode::UPGRADE_REQUIRED, NAUGHTY, "😳")
    } else if input.break_rule_9() {
        (StatusCode::IM_A_TEAPOT, NAUGHTY, "not a coffee brewer")
    } else {
        (StatusCode::OK, NICE, "that's a nice password")
    };

    (
        response_code,
        Json(GameValidation {
            result: result.to_string(),
            reason: reason.to_string(),
        }),
    )
}
