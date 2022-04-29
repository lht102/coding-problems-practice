use std::collections::HashMap;
use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn longest_palindrome(words: Vec<String>) -> i32 {
        let freq = words
            .iter()
            .fold(HashMap::<String, usize>::new(), |mut map, s| {
                *map.entry(s.clone()).or_default() += 1;
                map
            });
        let mut res = 0;
        let mut set = HashSet::<String>::new();
        let mut extra = 0;
        for k in freq.keys() {
            if !set.insert(k.clone()) {
                continue;
            }
            let chs = k.chars().collect::<Vec<_>>();
            if chs[0] == chs[1] {
                if let Some(&v) = freq.get(k) {
                    res += v / 2;
                    if v % 2 == 1 {
                        extra = 2;
                    }
                }
            } else {
                let s = [chs[1], chs[0]].iter().collect::<String>();
                res += freq.get(k).unwrap_or(&0).min(freq.get(&s).unwrap_or(&0));
                set.insert(s);
            }
        }
        (res * 4 + extra) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let words = vec![String::from("lc"), String::from("cl"), String::from("gg")];
        assert_eq!(Solution::longest_palindrome(words), 6);

        let words = vec![
            String::from("ab"),
            String::from("ty"),
            String::from("yt"),
            String::from("lc"),
            String::from("cl"),
            String::from("ab"),
        ];
        assert_eq!(Solution::longest_palindrome(words), 8);

        let words = vec![String::from("cc"), String::from("ll"), String::from("xx")];
        assert_eq!(Solution::longest_palindrome(words), 2);

        let words = vec![
            String::from("dd"),
            String::from("aa"),
            String::from("bb"),
            String::from("dd"),
            String::from("aa"),
            String::from("dd"),
            String::from("bb"),
            String::from("dd"),
            String::from("aa"),
            String::from("cc"),
            String::from("bb"),
            String::from("cc"),
            String::from("dd"),
            String::from("cc"),
        ];
        assert_eq!(Solution::longest_palindrome(words), 22);
    }
}
