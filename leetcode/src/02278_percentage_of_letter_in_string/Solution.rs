struct Solution;

impl Solution {
    pub fn percentage_letter(s: String, letter: char) -> i32 {
        (s.chars().filter(|&ch| ch == letter).count() as f64 / s.len() as f64 * 100.0) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let s = String::from("foobar");
        let letter = 'o';
        assert_eq!(Solution::percentage_letter(s, letter), 33);

        let s = String::from("jjjj");
        let letter = 'k';
        assert_eq!(Solution::percentage_letter(s, letter), 0);
    }
}
