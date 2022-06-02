struct Solution;

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let s = s
            .chars()
            .filter_map(|ch| {
                if ch.is_ascii_alphanumeric() {
                    Some(ch.to_ascii_lowercase())
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        let n = s.len();
        for i in 0..n / 2 {
            if s[i] != s[n - 1 - i] {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let s = String::from("A man, a plan, a canal: Panama");
        assert!(Solution::is_palindrome(s));

        let s = String::from("race a car");
        assert!(!Solution::is_palindrome(s));

        let s = String::from(" ");
        assert!(Solution::is_palindrome(s));
    }
}
