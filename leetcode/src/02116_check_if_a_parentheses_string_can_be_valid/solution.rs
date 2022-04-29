struct Solution;

impl Solution {
    pub fn can_be_valid(s: String, locked: String) -> bool {
        if s.len() % 2 == 1 {
            return false;
        }
        let s = s.chars().collect::<Vec<_>>();
        let locked = locked.chars().collect::<Vec<_>>();
        let mut open = 0;
        for i in 0..s.len() {
            if locked[i] == '1' && s[i] == ')' {
                open -= 1;
                if open < 0 {
                    return false;
                }
            } else {
                open += 1;
            }
        }
        let mut close = 0;
        for i in (0..s.len()).rev() {
            if locked[i] == '1' && s[i] == '(' {
                close -= 1;
                if close < 0 {
                    return false;
                }
            } else {
                close += 1;
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
        let s = String::from("))()))");
        let locked = String::from("010100");
        assert!(Solution::can_be_valid(s, locked));

        let s = String::from("()()");
        let locked = String::from("0000");
        assert!(Solution::can_be_valid(s, locked));

        let s = String::from(")");
        let locked = String::from("0");
        assert!(!Solution::can_be_valid(s, locked));
    }
}
