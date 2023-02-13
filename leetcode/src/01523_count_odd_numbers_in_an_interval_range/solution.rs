struct Solution;

impl Solution {
    pub fn count_odds(low: i32, high: i32) -> i32 {
        (high - low) / 2 + if low % 2 == 1 || high % 2 == 1 { 1 } else { 0 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let low = 3;
        let high = 7;
        assert_eq!(Solution::count_odds(low, high), 3);

        let low = 8;
        let high = 10;
        assert_eq!(Solution::count_odds(low, high), 1);
    }
}
