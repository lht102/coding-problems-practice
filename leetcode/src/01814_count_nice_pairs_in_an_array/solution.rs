use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn count_nice_pairs(nums: Vec<i32>) -> i32 {
        let m = 1_000_000_007;
        let mut res = 0;
        let mut freq: HashMap<i32, usize> = HashMap::new();
        for &n in &nums {
            if let Some(v) = freq.get(&(n - Self::rev(n))) {
                res += v;
                res %= m;
            }
            *freq.entry(n - Self::rev(n)).or_default() += 1;
        }
        res as i32
    }

    fn rev(n: i32) -> i32 {
        let mut res = 0;
        let mut n = n;
        while n > 0 {
            res = res * 10 + n % 10;
            n /= 10;
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let nums = vec![42, 11, 1, 97];
        assert_eq!(Solution::count_nice_pairs(nums), 2);

        let nums = vec![13, 10, 35, 24, 76];
        assert_eq!(Solution::count_nice_pairs(nums), 4);
    }
}
