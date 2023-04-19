struct Solution;

impl Solution {
    pub fn merge_alternately(word1: String, word2: String) -> String {
        word1
            .chars()
            .zip(word2.chars())
            .flat_map(|(ch1, ch2)| [ch1, ch2])
            .chain(word1.chars().skip(word2.len()))
            .chain(word2.chars().skip(word1.len()))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let word1 = String::from("abc");
        let word2 = String::from("pqr");
        assert_eq!(
            Solution::merge_alternately(word1, word2),
            String::from("apbqcr")
        );

        let word1 = String::from("ab");
        let word2 = String::from("pqrs");
        assert_eq!(
            Solution::merge_alternately(word1, word2),
            String::from("apbqrs")
        );

        let word1 = String::from("abcd");
        let word2 = String::from("pq");
        assert_eq!(
            Solution::merge_alternately(word1, word2),
            String::from("apbqcd")
        );
    }
}
