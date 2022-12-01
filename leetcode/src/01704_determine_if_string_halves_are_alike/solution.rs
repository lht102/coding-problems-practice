struct Solution;

impl Solution {
    pub fn halves_are_alike(s: String) -> bool {
        let n = s.len() / 2;
        Solution::count_vowels(&s[..n]) == Solution::count_vowels(&s[n..])
    }

    fn count_vowels(s: &str) -> usize {
        s.chars().filter(|&ch| "aeiouAEIOU".contains(ch)).count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let s = String::from("book");
        assert!(Solution::halves_are_alike(s));

        let s = String::from("textbook");
        assert!(!Solution::halves_are_alike(s));
    }
}
