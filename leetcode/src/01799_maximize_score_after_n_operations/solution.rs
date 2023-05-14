struct Solution;

impl Solution {
    pub fn max_score(nums: Vec<i32>) -> i32 {
        let mut dp = vec![None; 1 << nums.len()];
        Self::solve(&nums, 0, 0, &mut dp)
    }

    fn solve(nums: &[i32], i: usize, mask: usize, dp: &mut [Option<i32>]) -> i32 {
        if i * 2 == nums.len() {
            return 0;
        }
        if let Some(val) = dp[mask] {
            return val;
        }
        let mut res = 0;
        for (j, &num1) in nums.iter().enumerate() {
            for (k, &num2) in nums.iter().enumerate().skip(j + 1) {
                if mask & (1 << j) > 0 || mask & (1 << k) > 0 {
                    continue;
                }
                res = res.max(
                    Self::solve(nums, i + 1, mask | (1 << j) | (1 << k), dp)
                        + (i as i32 + 1) * Self::gcd(num1, num2),
                );
            }
        }
        dp[mask] = Some(res);
        res
    }

    fn gcd(a: i32, b: i32) -> i32 {
        let mut a = a;
        let mut b = b;
        while b != 0 {
            a %= b;
            std::mem::swap(&mut b, &mut a);
        }
        a
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let nums = vec![1, 2];
        assert_eq!(Solution::max_score(nums), 1);

        let nums = vec![3, 4, 6, 8];
        assert_eq!(Solution::max_score(nums), 11);

        let nums = vec![1, 2, 3, 4, 5, 6];
        assert_eq!(Solution::max_score(nums), 14);
    }
}
