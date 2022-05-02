struct Solution;

impl Solution {
    pub fn remove_digit(number: String, digit: char) -> String {
        let mut s = number.chars().collect::<Vec<_>>();
        for i in 1..s.len() {
            if s[i - 1] == digit && s[i] > digit {
                s.remove(i - 1);
                return s.iter().collect();
            }
        }
        let last = s.iter().rposition(|&ch| ch == digit).unwrap();
        s.remove(last);
        s.iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let number = String::from("123");
        let digit = '3';
        assert_eq!(Solution::remove_digit(number, digit), String::from("12"));

        let number = String::from("1231");
        let digit = '1';
        assert_eq!(Solution::remove_digit(number, digit), String::from("231"));

        let number = String::from("551");
        let digit = '5';
        assert_eq!(Solution::remove_digit(number, digit), String::from("51"));

        let number = String::from("133235");
        let digit = '3';
        assert_eq!(Solution::remove_digit(number, digit), String::from("13325"));
    }
}
