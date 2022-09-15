use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn partition_string(s: String) -> i32 {
        let mut st = HashSet::new();
        let mut res = 1;
        for ch in s.chars() {
            if st.contains(&ch) {
                st.clear();
                res += 1;
            }
            st.insert(ch);
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let s = String::from("abacaba");
        assert_eq!(Solution::partition_string(s), 4);

        let s = String::from("ssssss");
        assert_eq!(Solution::partition_string(s), 6);
    }
}
