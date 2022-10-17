use std::collections::HashSet;
use std::iter::FromIterator;

struct Solution;

impl Solution {
    pub fn check_if_pangram(sentence: String) -> bool {
        let set = HashSet::<char>::from_iter(sentence.chars());
        !('a'..='z').any(|ch| !set.contains(&ch))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let sentence = String::from("thequickbrownfoxjumpsoverthelazydog");
        assert!(Solution::check_if_pangram(sentence));

        let sentence = String::from("leetcode");
        assert!(!Solution::check_if_pangram(sentence));
    }
}
