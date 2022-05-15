struct Solution;

impl Solution {
    pub fn ways_to_split_array(nums: Vec<i32>) -> i32 {
        let mut p_sum = vec![0i64; nums.len() + 1];
        for i in 1..=nums.len() {
            p_sum[i] = p_sum[i - 1] + nums[i - 1] as i64;
        }
        let mut res = 0;
        for i in 1..nums.len() {
            let pre = p_sum[i];
            let suf = p_sum[nums.len()] - p_sum[i];
            if pre >= suf {
                res += 1;
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test() {
        let nums = vec![10, 4, -8, 7];
        assert_eq!(Solution::ways_to_split_array(nums), 2);

        let nums = vec![2, 3, 1, 0];
        assert_eq!(Solution::ways_to_split_array(nums), 2);
    }
}
