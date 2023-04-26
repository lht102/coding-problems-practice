struct Solution;

impl Solution {
    pub fn add_digits(num: i32) -> i32 {
        if num == 0 {
            0
        } else if num % 9 == 0 {
            9
        } else {
            num % 9
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let num = 38;
        assert_eq!(Solution::add_digits(num), 2);

        let num = 0;
        assert_eq!(Solution::add_digits(num), 0);
    }
}
