use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn rearrange_characters(s: String, target: String) -> i32 {
        let target_freq = target
            .chars()
            .fold(HashMap::<char, usize>::new(), |mut map, ch| {
                *map.entry(ch).or_default() += 1;
                map
            });
        let s_freq = s
            .chars()
            .fold(HashMap::<char, usize>::new(), |mut map, ch| {
                *map.entry(ch).or_default() += 1;
                map
            });
        let mut res = usize::MAX;
        for (ch, cnt) in &target_freq {
            if let Some(c) = s_freq.get(ch) {
                res = res.min(c / cnt);
            } else {
                return 0;
            }
        }
        if res == usize::MAX {
            0
        } else {
            res as i32
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let s = String::from("ilovecodingonleetcode");
        let target = String::from("code");
        assert_eq!(Solution::rearrange_characters(s, target), 2);

        let s = String::from("abcba");
        let target = String::from("abc");
        assert_eq!(Solution::rearrange_characters(s, target), 1);

        let s = String::from("abbaccaddaeea");
        let target = String::from("aaaa");
        assert_eq!(Solution::rearrange_characters(s, target), 1);

        let s = String::from("abc");
        let target = String::from("abcd");
        assert_eq!(Solution::rearrange_characters(s, target), 0);
    }
}
