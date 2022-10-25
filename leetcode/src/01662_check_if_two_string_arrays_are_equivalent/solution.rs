struct Solution;

impl Solution {
    pub fn array_strings_are_equal(word1: Vec<String>, word2: Vec<String>) -> bool {
        word1.join("") == word2.join("")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let word1 = vec![String::from("ab"), String::from("c")];
        let word2 = vec![String::from("a"), String::from("bc")];
        assert!(Solution::array_strings_are_equal(word1, word2));

        let word1 = vec![String::from("a"), String::from("cb")];
        let word2 = vec![String::from("ab"), String::from("c")];
        assert!(!Solution::array_strings_are_equal(word1, word2));

        let word1 = vec![String::from("abc"), String::from("d"), String::from("defg")];
        let word2 = vec![String::from("abcddefg")];
        assert!(Solution::array_strings_are_equal(word1, word2));
    }
}
