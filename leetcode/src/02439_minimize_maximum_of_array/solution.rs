struct Solution;

impl Solution {
    pub fn minimize_array_value(nums: Vec<i32>) -> i32 {
        nums.iter()
            .scan(0, |acc, &num| {
                *acc += num as i64;
                Some(*acc)
            })
            .enumerate()
            .map(|(i, prefix_sum)| (prefix_sum + i as i64) / (i + 1) as i64)
            .max()
            .unwrap() as _
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let nums = vec![3, 7, 1, 6];
        assert_eq!(Solution::minimize_array_value(nums), 5);

        let nums = vec![10, 1];
        assert_eq!(Solution::minimize_array_value(nums), 10);

        let nums = vec![13, 13, 20, 0, 8, 9, 9];
        assert_eq!(Solution::minimize_array_value(nums), 16);
    }
}
