struct Solution;

impl Solution {
    pub fn remove_duplicates(s: String, k: i32) -> String {
        let mut st = vec![('\0', 0)];
        st.push(('\0', 0));
        for ch in s.chars() {
            if let Some(last) = st.last_mut() {
                if last.0 == ch {
                    last.1 += 1;
                    if last.1 == k as usize {
                        st.pop();
                    }
                } else {
                    st.push((ch, 1));
                }
            }
        }
        let mut res = String::new();
        for t in st {
            res.push_str(&std::iter::repeat_n(t.0, t.1).collect::<String>());
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let s = String::from("abcd");
        let k = 2;
        assert_eq!(Solution::remove_duplicates(s, k), String::from("abcd"));

        let s = String::from("deeedbbcccbdaa");
        let k = 3;
        assert_eq!(Solution::remove_duplicates(s, k), String::from("aa"));

        let s = String::from("pbbcggttciiippooaais");
        let k = 2;
        assert_eq!(Solution::remove_duplicates(s, k), String::from("ps"));
    }
}
