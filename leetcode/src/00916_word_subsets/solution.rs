use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn word_subsets(words1: Vec<String>, words2: Vec<String>) -> Vec<String> {
        let counter =
            words2
                .into_iter()
                .fold(HashMap::<char, usize>::with_capacity(26), |mut map, w| {
                    let tmp = Solution::to_counter(&w);
                    for ch in 'a'..'z' {
                        let entry = map.entry(ch).or_default();
                        *entry = (*entry).max(*tmp.get(&ch).unwrap_or(&0));
                    }
                    map
                });
        words1
            .into_iter()
            .filter(|w| {
                let tmp = Solution::to_counter(w);
                for ch in 'a'..'z' {
                    if *tmp.get(&ch).unwrap_or(&0) < *counter.get(&ch).unwrap_or(&0) {
                        return false;
                    }
                }
                true
            })
            .collect()
    }

    fn to_counter(s: &str) -> HashMap<char, usize> {
        s.chars()
            .fold(HashMap::<char, usize>::with_capacity(26), |mut map, ch| {
                *map.entry(ch).or_default() += 1;
                map
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let words1 = vec![
            String::from("amazon"),
            String::from("apple"),
            String::from("facebook"),
            String::from("google"),
            String::from("leetcode"),
        ];
        let words2 = vec![String::from("e"), String::from("o")];
        assert_eq!(
            Solution::word_subsets(words1, words2),
            vec![
                String::from("facebook"),
                String::from("google"),
                String::from("leetcode")
            ]
        );

        let words1 = vec![
            String::from("amazon"),
            String::from("apple"),
            String::from("facebook"),
            String::from("google"),
            String::from("leetcode"),
        ];
        let words2 = vec![String::from("l"), String::from("e")];
        assert_eq!(
            Solution::word_subsets(words1, words2),
            vec![
                String::from("apple"),
                String::from("google"),
                String::from("leetcode")
            ]
        );
    }
}
