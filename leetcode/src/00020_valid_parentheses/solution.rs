use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let pairs = HashMap::from([(')', '('), (']', '['), ('}', '{')]);
        let mut st = Vec::new();
        for ch in s.chars() {
            match pairs.get(&ch) {
                Some(&open) => match st.pop() {
                    Some(top) if top == open => {}
                    _ => return false,
                },
                None => st.push(ch),
            }
        }
        st.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let s = String::from("()");
        assert!(Solution::is_valid(s));

        let s = String::from("()[]{}");
        assert!(Solution::is_valid(s));

        let s = String::from("(]");
        assert!(!Solution::is_valid(s));

        let s = String::from("(((");
        assert!(!Solution::is_valid(s));
    }
}
