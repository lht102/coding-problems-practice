struct Solution;

impl Solution {
    pub fn max_vowels(s: String, k: i32) -> i32 {
        let s = s.as_bytes();
        let k = k as usize;
        let mut cnt = s.iter().take(k).filter(|&&ch| Self::is_vowel(ch)).count();
        let mut res = cnt;
        for end in k..s.len() {
            let start = end - k;
            if Self::is_vowel(s[end]) {
                cnt += 1;
            }
            if Self::is_vowel(s[start]) {
                cnt -= 1;
            }
            res = res.max(cnt);
        }
        res as _
    }

    fn is_vowel(ch: u8) -> bool {
        ch == b'a' || ch == b'e' || ch == b'i' || ch == b'o' || ch == b'u'
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let s = String::from("abciiidef");
        let k = 3;
        assert_eq!(Solution::max_vowels(s, k), 3);

        let s = String::from("aeiou");
        let k = 2;
        assert_eq!(Solution::max_vowels(s, k), 2);

        let s = String::from("leetcode");
        let k = 3;
        assert_eq!(Solution::max_vowels(s, k), 2);
    }
}
