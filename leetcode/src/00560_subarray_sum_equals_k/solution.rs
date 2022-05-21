use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        let mut res = 0;
        let mut sum = 0;
        let mut freq = HashMap::from([(0, 1)]);
        for num in &nums {
            sum += num;
            if let Some(cnt) = freq.get(&(sum - k)) {
                res += cnt;
            }
            *freq.entry(sum).or_default() += 1;
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let nums = vec![1, 1, 1];
        let k = 2;
        assert_eq!(Solution::subarray_sum(nums, k), 2);

        let nums = vec![1, 2, 3];
        let k = 3;
        assert_eq!(Solution::subarray_sum(nums, k), 2);
    }
}
