struct Solution;

impl Solution {
    pub fn minimal_k_sum(nums: Vec<i32>, k: i32) -> i64 {
        let mut nums = nums;
        let mut k = k;
        nums.push(0);
        nums.sort_unstable();
        let mut res = 0;
        for i in 1..nums.len() {
            if nums[i - 1] == nums[i] {
                continue;
            }
            let right_bound = (nums[i - 1] + k).min(nums[i] - 1);
            res += Self::sum_range(nums[i - 1] + 1, right_bound);
            k -= right_bound - nums[i - 1];
            if k == 0 {
                break;
            }
        }
        res + if k > 0 {
            Self::sum_range(nums[nums.len() - 1] + 1, nums[nums.len() - 1] + k)
        } else {
            0
        }
    }

    fn sum_range(lo: i32, hi: i32) -> i64 {
        let lo = lo as i64;
        let hi = hi as i64;
        hi * (hi + 1) / 2 - (lo - 1) * lo / 2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let nums = vec![1, 4, 25, 10, 25];
        let k = 2;
        assert_eq!(Solution::minimal_k_sum(nums, k), 5);

        let nums = vec![5, 6];
        let k = 6;
        assert_eq!(Solution::minimal_k_sum(nums, k), 25);
    }
}
