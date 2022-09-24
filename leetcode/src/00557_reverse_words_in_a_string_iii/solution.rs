struct Solution;

impl Solution {
    pub fn reverse_words(s: String) -> String {
        s.split_whitespace()
            .map(|w| w.chars().rev().collect::<String>())
            .collect::<Vec<_>>()
            .join(" ")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let s = String::from("Let's take LeetCode contest");
        assert_eq!(
            Solution::reverse_words(s),
            String::from("s'teL ekat edoCteeL tsetnoc")
        );

        let s = String::from("God Ding");
        assert_eq!(Solution::reverse_words(s), String::from("doG gniD"));
    }
}
