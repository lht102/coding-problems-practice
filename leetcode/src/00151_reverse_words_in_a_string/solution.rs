struct Solution;

impl Solution {
    pub fn reverse_words(s: String) -> String {
        s.split_whitespace().rev().collect::<Vec<_>>().join(" ")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let s = String::from("the sky is blue");
        assert_eq!(Solution::reverse_words(s), String::from("blue is sky the"));

        let s = String::from("  hello world  ");
        assert_eq!(Solution::reverse_words(s), String::from("world hello"));

        let s = String::from("a good   example");
        assert_eq!(Solution::reverse_words(s), String::from("example good a"));
    }
}
