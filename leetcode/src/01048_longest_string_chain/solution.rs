use std::collections::HashMap;
use std::collections::HashSet;
use std::iter::FromIterator;

struct Solution;

impl Solution {
    pub fn longest_str_chain(words: Vec<String>) -> i32 {
        let mut word_set = HashSet::from_iter(words.iter().cloned());
        let mut dp = HashMap::new();
        words
            .iter()
            .map(|w| Solution::solve(w, &mut word_set, &mut dp))
            .max()
            .unwrap() as i32
    }

    fn solve(word: &str, word_set: &mut HashSet<String>, dp: &mut HashMap<String, usize>) -> usize {
        if let Some(&len) = dp.get(word) {
            return len;
        }
        let mut res = 1;
        for i in 0..word.len() {
            let pred = [&word[..i], &word[i + 1..]].join("");
            if word_set.contains(&pred) {
                res = res.max(Solution::solve(&pred, word_set, dp) + 1);
            }
        }
        dp.insert(word.to_owned(), res);
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let words = vec![
            String::from("a"),
            String::from("b"),
            String::from("ba"),
            String::from("bca"),
            String::from("bda"),
            String::from("bdca"),
        ];
        assert_eq!(Solution::longest_str_chain(words), 4);

        let words = vec![
            String::from("xbc"),
            String::from("pcxbcf"),
            String::from("xb"),
            String::from("cxbc"),
            String::from("pcxbc"),
        ];
        assert_eq!(Solution::longest_str_chain(words), 5);

        let words = vec![String::from("abcd"), String::from("dbqca")];
        assert_eq!(Solution::longest_str_chain(words), 1);
    }
}
