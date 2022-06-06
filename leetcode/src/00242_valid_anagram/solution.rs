use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }
        let mut freq = s.chars().fold(HashMap::<char, i32>::new(), |mut map, ch| {
            *map.entry(ch).or_default() += 1;
            map
        });
        for ch in t.chars() {
            *freq.entry(ch).or_default() -= 1;
        }
        !freq.values().any(|&cnt| cnt != 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let s = String::from("anagram");
        let t = String::from("nagaram");
        assert!(Solution::is_anagram(s, t));

        let s = String::from("rat");
        let t = String::from("car");
        assert!(!Solution::is_anagram(s, t));
    }
}
