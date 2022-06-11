struct Solution;

impl Solution {
    pub fn strong_password_checker_ii(password: String) -> bool {
        password.len() >= 8
            && Solution::contains_lowercase_letter(&password)
            && Solution::contains_uppercase_letter(&password)
            && Solution::contains_digit(&password)
            && Solution::contains_special_character(&password)
            && !Solution::contains_adjacent_repeated_characters(&password)
    }

    fn contains_lowercase_letter(s: &str) -> bool {
        s.chars().any(|ch| ch.is_ascii_lowercase())
    }

    fn contains_uppercase_letter(s: &str) -> bool {
        s.chars().any(|ch| ch.is_ascii_uppercase())
    }

    fn contains_digit(s: &str) -> bool {
        s.chars().any(|ch| ch.is_ascii_digit())
    }

    fn contains_special_character(s: &str) -> bool {
        s.chars().any(|ch| "!@#$%^&*()-+".contains(ch))
    }

    fn contains_adjacent_repeated_characters(s: &str) -> bool {
        s.chars()
            .zip(s.chars().skip(1))
            .any(|(ch1, ch2)| ch1 == ch2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let password = String::from("IloveLe3tcode!");
        assert!(Solution::strong_password_checker_ii(password));

        let password = String::from("Me+You--IsMyDream");
        assert!(!Solution::strong_password_checker_ii(password));

        let password = String::from("1aB!");
        assert!(!Solution::strong_password_checker_ii(password));

        let password = String::from("IloveLe3tcode!!");
        assert!(!Solution::strong_password_checker_ii(password));
    }
}
