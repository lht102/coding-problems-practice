use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn find_duplicate(paths: Vec<String>) -> Vec<Vec<String>> {
        paths
            .into_iter()
            .fold(HashMap::<String, Vec<String>>::new(), |mut map, path| {
                let words = path.split_whitespace().collect::<Vec<_>>();
                for w in words.iter().skip(1) {
                    let i = w.rfind('(').unwrap();
                    let filepath = vec![words[0], &w[..i]].join("/");
                    let content = w[i + 1..w.len() - 1].to_owned();
                    map.entry(content).or_default().push(filepath);
                }
                map
            })
            .into_values()
            .filter(|v| v.len() > 1)
            .collect()
    }
}
