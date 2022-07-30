use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn find_and_replace_pattern(words: Vec<String>, pattern: String) -> Vec<String> {
        let p = Solution::to_pattern(&pattern);
        words
            .into_iter()
            .filter(|w| p == Solution::to_pattern(w))
            .collect()
    }

    fn to_pattern(s: &str) -> String {
        let map = s
            .chars()
            .fold(HashMap::<char, usize>::new(), |mut map, ch| {
                let sz = map.len();
                map.entry(ch).or_insert(sz);
                map
            });
        s.chars()
            .map(|ch| (b'a' + *map.get(&ch).unwrap() as u8) as char)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let words = vec![
            String::from("abc"),
            String::from("deq"),
            String::from("mee"),
            String::from("aqq"),
            String::from("dkd"),
            String::from("ccc"),
        ];
        let pattern = String::from("abb");
        assert_eq!(
            Solution::find_and_replace_pattern(words, pattern),
            vec![String::from("mee"), String::from("aqq")]
        );

        let words = vec![String::from("a"), String::from("b"), String::from("c")];
        let pattern = String::from("a");
        assert_eq!(
            Solution::find_and_replace_pattern(words, pattern),
            vec![String::from("a"), String::from("b"), String::from("c")]
        );
    }
}
