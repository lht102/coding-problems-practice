struct Solution;

impl Solution {
    pub fn is_power_of_three(n: i32) -> bool {
        n > 0 && 3_i32.pow(19) % n == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let n = 27;
        assert!(Solution::is_power_of_three(n));

        let n = 0;
        assert!(!Solution::is_power_of_three(n));

        let n = -1;
        assert!(!Solution::is_power_of_three(n));
    }
}
