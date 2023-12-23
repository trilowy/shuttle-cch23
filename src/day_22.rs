use std::collections::HashMap;

pub async fn task_1(body: String) -> String {
    let result = unique(&body);

    "🎁".repeat(result)
}

fn unique(body: &str) -> usize {
    let count = body.split_whitespace().fold(HashMap::new(), |mut acc, e| {
        if let Some(value) = acc.get_mut(e) {
            *value += 1;
        } else {
            acc.insert(e, 1);
        }
        acc
    });

    count
        .into_iter()
        .find(|(_, value)| *value == 1)
        .map(|(key, _)| key)
        .unwrap()
        .parse()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use crate::day_22::unique;

    #[test]
    fn test_task_1() {
        let body = "888
77
888
22
77
";

        assert_eq!(unique(body), 22);
    }
}
