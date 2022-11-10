struct Solution;

impl Solution {
    pub fn remove_duplicates(s: String) -> String {
        s.chars()
            .fold(Vec::new(), |mut st, ch| {
                match st.last() {
                    Some(&top) if top == ch => _ = st.pop(),
                    _ => st.push(ch),
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
        let s = String::from("abbaca");
        assert_eq!(Solution::remove_duplicates(s), String::from("ca"));

        let s = String::from("azxxzy");
        assert_eq!(Solution::remove_duplicates(s), String::from("ay"));
    }
}
