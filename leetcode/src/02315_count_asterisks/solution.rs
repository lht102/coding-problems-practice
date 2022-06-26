struct Solution;

impl Solution {
    pub fn count_asterisks(s: String) -> i32 {
        s.split('|')
            .step_by(2)
            .map(|s| s.chars().filter(|&ch| ch == '*').count())
            .sum::<usize>() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let s = String::from("l|*e*et|c**o|*de|");
        assert_eq!(Solution::count_asterisks(s), 2);

        let s = String::from("iamprogrammer");
        assert_eq!(Solution::count_asterisks(s), 0);

        let s = String::from("yo|uar|e**|b|e***au|tifu|l");
        assert_eq!(Solution::count_asterisks(s), 5);
    }
}
