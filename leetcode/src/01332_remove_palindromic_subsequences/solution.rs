struct Solution;

impl Solution {
    pub fn remove_palindrome_sub(s: String) -> i32 {
        if s.is_empty() {
            0
        } else if Solution::is_p(&s.chars().collect::<Vec<_>>(), 0, s.len() - 1) {
            1
        } else {
            2
        }
    }

    fn is_p(s: &[char], i: usize, j: usize) -> bool {
        let mut i = i;
        let mut j = j;
        while i < j {
            if s[i] != s[j] {
                return false;
            }
            i += 1;
            j -= 1;
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let s = String::from("ababa");
        assert_eq!(Solution::remove_palindrome_sub(s), 1);

        let s = String::from("abb");
        assert_eq!(Solution::remove_palindrome_sub(s), 2);

        let s = String::from("baabb");
        assert_eq!(Solution::remove_palindrome_sub(s), 2);
    }
}
