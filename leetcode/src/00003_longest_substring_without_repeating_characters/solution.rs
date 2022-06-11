use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let s = s.chars().collect::<Vec<_>>();
        let mut ch_to_idx: HashMap<char, usize> = HashMap::new();
        let mut start = 0;
        let mut res = 0;
        for (end, &ch) in s.iter().enumerate() {
            if let Some(idx) = ch_to_idx.get(&ch) {
                start = start.max(idx + 1);
            }
            ch_to_idx.insert(ch, end);
            res = res.max(end + 1 - start);
        }
        res as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let s = String::from("abcabcbb");
        assert_eq!(Solution::length_of_longest_substring(s), 3);

        let s = String::from("bbbbb");
        assert_eq!(Solution::length_of_longest_substring(s), 1);

        let s = String::from("pwwkew");
        assert_eq!(Solution::length_of_longest_substring(s), 3);
    }
}
