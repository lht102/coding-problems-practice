use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn buddy_strings(s: String, goal: String) -> bool {
        let freq1 = s
            .chars()
            .fold(HashMap::<char, usize>::new(), |mut map, ch| {
                *map.entry(ch).or_default() += 1;
                map
            });
        let freq2 = goal
            .chars()
            .fold(HashMap::<char, usize>::new(), |mut map, ch| {
                *map.entry(ch).or_default() += 1;
                map
            });
        match s.chars().zip(goal.chars()).filter(|&(a, b)| a != b).count() {
            0 => freq1.values().any(|&cnt| cnt > 1),
            2 => freq1 == freq2,
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let s = String::from("ab");
        let goal = String::from("ba");
        assert!(Solution::buddy_strings(s, goal));

        let s = String::from("ab");
        let goal = String::from("ab");
        assert!(!Solution::buddy_strings(s, goal));

        let s = String::from("aa");
        let goal = String::from("aa");
        assert!(Solution::buddy_strings(s, goal));

        let s = String::from("abcaa");
        let goal = String::from("abcbb");
        assert!(!Solution::buddy_strings(s, goal));
    }
}
