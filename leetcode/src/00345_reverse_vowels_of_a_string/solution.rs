struct Solution;

impl Solution {
    pub fn reverse_vowels(s: String) -> String {
        let mut s = s.chars().collect::<Vec<_>>();
        let n = s.len();
        let (mut lo, mut hi) = (0, n - 1);
        while lo < hi {
            while lo < n && !Solution::is_vowel(s[lo]) {
                lo += 1;
            }
            while hi > 0 && !Solution::is_vowel(s[hi]) {
                hi -= 1;
            }
            if lo < hi {
                s.swap(lo, hi);
                lo += 1;
                if hi == 0 {
                    break;
                }
                hi -= 1;
            }
        }
        s.into_iter().collect()
    }

    fn is_vowel(ch: char) -> bool {
        ch == 'a'
            || ch == 'A'
            || ch == 'e'
            || ch == 'E'
            || ch == 'i'
            || ch == 'I'
            || ch == 'o'
            || ch == 'O'
            || ch == 'u'
            || ch == 'U'
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let s = String::from("hello");
        assert_eq!(Solution::reverse_vowels(s), String::from("holle"));

        let s = String::from("leetcode");
        assert_eq!(Solution::reverse_vowels(s), String::from("leotcede"));
    }
}
