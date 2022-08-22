struct Solution;

impl Solution {
    pub fn is_power_of_four(n: i32) -> bool {
        n > 0 && n & (n - 1) == 0 && 0b1010101010101010101010101010101 & n == n
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let n = 16;
        assert!(Solution::is_power_of_four(n));

        let n = 5;
        assert!(!Solution::is_power_of_four(n));

        let n = 1;
        assert!(Solution::is_power_of_four(n));
    }
}
