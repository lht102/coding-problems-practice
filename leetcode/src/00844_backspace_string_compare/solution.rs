struct Solution;

impl Solution {
    pub fn backspace_compare(s: String, t: String) -> bool {
        Self::after_backspace(&s) == Self::after_backspace(&t)
    }

    fn after_backspace(s: &str) -> String {
        let mut st = Vec::new();
        for ch in s.chars() {
            if ch == '#' {
                st.pop();
            } else {
                st.push(ch);
            }
        }
        st.iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let s = String::from("ab#c");
        let t = String::from("ad#c");
        assert!(Solution::backspace_compare(s, t));

        let s = String::from("ab##");
        let t = String::from("c#d#");
        assert!(Solution::backspace_compare(s, t));

        let s = String::from("a#c");
        let t = String::from("b");
        assert!(!Solution::backspace_compare(s, t));
    }
}
