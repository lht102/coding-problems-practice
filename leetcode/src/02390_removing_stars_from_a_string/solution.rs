struct Solution;

impl Solution {
    pub fn remove_stars(s: String) -> String {
        s.chars()
            .fold(Vec::new(), |mut st, ch| {
                if ch == '*' {
                    st.pop();
                } else {
                    st.push(ch);
                }
                st
            })
            .into_iter()
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let s = String::from("leet**cod*e");
        assert_eq!(Solution::remove_stars(s), String::from("lecoe"));

        let s = String::from("erase*****");
        assert_eq!(Solution::remove_stars(s), String::from(""));
    }
}
