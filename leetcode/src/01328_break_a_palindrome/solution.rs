struct Solution;

impl Solution {
    pub fn break_palindrome(palindrome: String) -> String {
        if palindrome.len() < 2 {
            return "".to_owned();
        }
        palindrome
            .char_indices()
            .take(palindrome.len() / 2)
            .find_map(|(i, ch)| {
                (ch != 'a').then_some(format!("{}a{}", &palindrome[0..i], &palindrome[i + 1..]))
            })
            .unwrap_or(format!("{}b", &palindrome[0..palindrome.len() - 1]))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let palindrome = String::from("abccba");
        assert_eq!(
            Solution::break_palindrome(palindrome),
            String::from("aaccba")
        );

        let palindrome = String::from("a");
        assert_eq!(Solution::break_palindrome(palindrome), String::from(""));
    }
}
