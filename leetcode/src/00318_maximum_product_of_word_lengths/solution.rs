struct Solution;

impl Solution {
    pub fn max_product(words: Vec<String>) -> i32 {
        let n = words.len();
        let bits = words
            .iter()
            .map(|w| w.chars().fold(0, |mask, ch| mask | 1 << (ch as u8 - b'a')))
            .collect::<Vec<_>>();
        let mut res = 0;
        for i in 0..n {
            for j in 0..n {
                if bits[i] & bits[j] == 0 {
                    res = res.max(words[i].len() * words[j].len());
                }
            }
        }
        res as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let words = vec![
            String::from("abcw"),
            String::from("baz"),
            String::from("foo"),
            String::from("bar"),
            String::from("xtfn"),
            String::from("abcdef"),
        ];
        assert_eq!(Solution::max_product(words), 16);

        let words = vec![
            String::from("a"),
            String::from("ab"),
            String::from("abc"),
            String::from("d"),
            String::from("cd"),
            String::from("bcd"),
            String::from("abcd"),
        ];
        assert_eq!(Solution::max_product(words), 4);

        let words = vec![
            String::from("a"),
            String::from("aa"),
            String::from("aaa"),
            String::from("aaaa"),
        ];
        assert_eq!(Solution::max_product(words), 0);
    }
}
