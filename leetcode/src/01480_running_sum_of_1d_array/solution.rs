struct Solution;

impl Solution {
    pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
        nums.iter()
            .scan(0, |acc, num| {
                *acc += num;
                Some(*acc)
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let nums = vec![1, 2, 3, 4];
        assert_eq!(Solution::running_sum(nums), vec![1, 3, 6, 10]);

        let nums = vec![1, 1, 1, 1, 1];
        assert_eq!(Solution::running_sum(nums), vec![1, 2, 3, 4, 5]);

        let nums = vec![3, 1, 2, 10, 1];
        assert_eq!(Solution::running_sum(nums), vec![3, 4, 6, 16, 17]);
    }
}
