use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn subarrays_div_by_k(nums: Vec<i32>, k: i32) -> i32 {
        let mut psum = HashMap::from([(0, 1)]);
        let mut sum = 0;
        let mut res = 0;
        for &num in &nums {
            sum = (sum + (num + k) % k + k) % k;
            if let Some(cnt) = psum.get(&sum) {
                res += cnt;
            }
            *psum.entry(sum).or_default() += 1;
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let nums = vec![4, 5, 0, -2, -3, 1];
        let k = 5;
        assert_eq!(Solution::subarrays_div_by_k(nums, k), 7);

        let nums = vec![5];
        let k = 9;
        assert_eq!(Solution::subarrays_div_by_k(nums, k), 0);
    }
}
