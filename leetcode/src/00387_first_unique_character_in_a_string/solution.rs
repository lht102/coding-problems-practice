use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn first_uniq_char(s: String) -> i32 {
        let freq = s
            .chars()
            .fold(HashMap::<char, usize>::new(), |mut map, ch| {
                *map.entry(ch).or_default() += 1;
                map
            });
        s.char_indices()
            .find(|(_, ch)| *freq.get(ch).unwrap() == 1)
            .map_or(-1, |(i, _)| i as i32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let s = String::from("leetcode");
        assert_eq!(Solution::first_uniq_char(s), 0);

        let s = String::from("loveleetcode");
        assert_eq!(Solution::first_uniq_char(s), 2);

        let s = String::from("aabb");
        assert_eq!(Solution::first_uniq_char(s), -1);
    }
}
