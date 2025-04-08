struct Solution;

impl Solution {
    pub fn make_good(s: String) -> String {
        let mut st: Vec<char> = Vec::new();
        for ch in s.chars() {
            if st.last().is_some_and(|&top| {
                top.eq_ignore_ascii_case(&ch) && top != ch
            }) {
                st.pop();
            } else {
                st.push(ch)
            }
        }
        st.into_iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let s = String::from("leEeetcode");
        assert_eq!(Solution::make_good(s), String::from("leetcode"));

        let s = String::from("abBAcC");
        assert_eq!(Solution::make_good(s), String::from(""));

        let s = String::from("s");
        assert_eq!(Solution::make_good(s), String::from("s"));

        let s = String::from("mC");
        assert_eq!(Solution::make_good(s), String::from("mC"));
    }
}
