use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn greatest_letter(s: String) -> String {
        let mut seen = HashSet::new();
        for ch in s.chars() {
            if ch.is_lowercase() {
                seen.insert(ch.to_ascii_lowercase());
            }
        }
        let mut res = '\0';
        for ch in s.chars() {
            if ch.is_uppercase() && seen.contains(&ch.to_ascii_lowercase()) {
                res = res.max(ch);
            }
        }
        if res == '\0' {
            String::new()
        } else {
            res.to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let s = String::from("lEeTcOdE");
        assert_eq!(Solution::greatest_letter(s), String::from("E"));

        let s = String::from("arRAzFif");
        assert_eq!(Solution::greatest_letter(s), String::from("R"));

        let s = String::from("AbCdEfGhIjK");
        assert_eq!(Solution::greatest_letter(s), String::from(""));
    }
}
