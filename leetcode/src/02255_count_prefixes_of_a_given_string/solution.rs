struct Solution;

impl Solution {
    pub fn count_prefixes(words: Vec<String>, s: String) -> i32 {
        words.iter().fold(0, |res, w| {
            if s.starts_with(w) {
                return res + 1;
            }
            res
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let words = vec![
            String::from("a"),
            String::from("b"),
            String::from("c"),
            String::from("ab"),
            String::from("bc"),
            String::from("abc"),
        ];
        let s = String::from("abc");
        assert_eq!(Solution::count_prefixes(words, s), 3);

        let words = vec![String::from("a"), String::from("a")];
        let s = String::from("aa");
        assert_eq!(Solution::count_prefixes(words, s), 2);
    }
}
