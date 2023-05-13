struct Solution;

const M: i32 = 1_000_000_007;

impl Solution {
    pub fn count_good_strings(low: i32, high: i32, zero: i32, one: i32) -> i32 {
        let low = low as usize;
        let high = high as usize;
        let zero = zero as usize;
        let one = one as usize;
        let mut dp = vec![None; high + 1];
        (low..=high).fold(0, |acc, x| (acc + Self::solve(x, zero, one, &mut dp)) % M)
    }

    fn solve(high: usize, zero: usize, one: usize, dp: &mut [Option<i32>]) -> i32 {
        if high == 0 {
            return 1;
        }
        if let Some(val) = dp[high] {
            return val;
        }
        let cnt = (if high >= zero {
            Self::solve(high - zero, zero, one, dp)
        } else {
            0
        } + if high >= one {
            Self::solve(high - one, zero, one, dp)
        } else {
            0
        }) % M;
        dp[high] = Some(cnt);
        cnt
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let low = 3;
        let high = 3;
        let zero = 1;
        let one = 1;
        assert_eq!(Solution::count_good_strings(low, high, zero, one), 8);

        let low = 2;
        let high = 3;
        let zero = 1;
        let one = 2;
        assert_eq!(Solution::count_good_strings(low, high, zero, one), 5);

        let low = 1;
        let high = 100000;
        let zero = 1;
        let one = 1;
        assert_eq!(
            Solution::count_good_strings(low, high, zero, one),
            215447031
        );
    }
}
