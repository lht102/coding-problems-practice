use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn is_scramble(s1: String, s2: String) -> bool {
        let mut dp = HashMap::new();
        Self::solve(&s1, &s2, &mut dp)
    }

    fn solve<'a>(s1: &'a str, s2: &'a str, dp: &mut HashMap<(&'a str, &'a str), bool>) -> bool {
        if s1 == s2 {
            return true;
        }
        if !Self::is_anagram(s1, s2) {
            return false;
        }
        let key = (s1, s2);
        if let Some(&val) = dp.get(&key) {
            return val;
        }
        let n = s1.len();
        for len in 1..n {
            let pre1 = &s1[..len];
            let suf1 = &s1[len..];
            if Self::solve(pre1, &s2[..len], dp) && Self::solve(suf1, &s2[len..], dp)
                || Self::solve(pre1, &s2[n - len..], dp) && Self::solve(suf1, &s2[..n - len], dp)
            {
                dp.insert(key, true);
                return true;
            }
        }
        dp.insert(key, false);
        false
    }

    pub fn is_anagram(s1: &str, s2: &str) -> bool {
        let mut freq = [0; 26];
        let get_index = |x: u8| (x - b'a') as usize;
        for ch in s1.bytes() {
            freq[get_index(ch)] += 1;
        }
        for ch in s2.bytes() {
            freq[get_index(ch)] -= 1;
        }
        !freq.into_iter().any(|cnt| cnt != 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let s1 = String::from("great");
        let s2 = String::from("rgeat");
        assert!(Solution::is_scramble(s1, s2));

        let s1 = String::from("abcde");
        let s2 = String::from("caebd");
        assert!(!Solution::is_scramble(s1, s2));

        let s1 = String::from("a");
        let s2 = String::from("a");
        assert!(Solution::is_scramble(s1, s2));
    }
}
