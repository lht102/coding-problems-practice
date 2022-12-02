use std::collections::HashMap;
use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn close_strings(word1: String, word2: String) -> bool {
        let counter = |w: &str| {
            w.chars()
                .fold(HashMap::<char, usize>::new(), |mut map, ch| {
                    *map.entry(ch).or_default() += 1;
                    map
                })
        };
        let freq1 = counter(&word1);
        let freq2 = counter(&word2);
        let mut cnt1 = freq1.values().collect::<Vec<_>>();
        cnt1.sort_unstable();
        let mut cnt2 = freq2.values().collect::<Vec<_>>();
        cnt2.sort_unstable();
        let set1 = freq1.keys().collect::<HashSet<_>>();
        let set2 = freq2.keys().collect::<HashSet<_>>();
        cnt1 == cnt2 && set1 == set2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let word1 = String::from("abc");
        let word2 = String::from("bca");
        assert!(Solution::close_strings(word1, word2));

        let word1 = String::from("a");
        let word2 = String::from("aa");
        assert!(!Solution::close_strings(word1, word2));

        let word1 = String::from("cabbba");
        let word2 = String::from("abbccc");
        assert!(Solution::close_strings(word1, word2));
    }
}
