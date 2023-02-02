use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn is_alien_sorted(words: Vec<String>, order: String) -> bool {
        let lookup_table = order.chars().zip('a'..='z').collect::<HashMap<_, _>>();
        words
            .iter()
            .map(|word| Solution::transform(word, &lookup_table))
            .collect::<Vec<_>>()
            .windows(2)
            .all(|w| w[0] <= w[1])
    }

    fn transform(word: &str, lookup_table: &HashMap<char, char>) -> String {
        word.chars()
            .map(|ch| lookup_table.get(&ch).unwrap())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let words = vec![String::from("hello"), String::from("leetcode")];
        let order = String::from("hlabcdefgijkmnopqrstuvwxyz");
        assert!(Solution::is_alien_sorted(words, order));

        let words = vec![
            String::from("word"),
            String::from("world"),
            String::from("row"),
        ];
        let order = String::from("worldabcefghijkmnpqstuvxyz");
        assert!(!Solution::is_alien_sorted(words, order));

        let words = vec![String::from("apple"), String::from("app")];
        let order = String::from("abcdefghijklmnopqrstuvwxyz");
        assert!(!Solution::is_alien_sorted(words, order));
    }
}
