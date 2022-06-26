struct Solution;

impl Solution {
    pub fn maximum_xor(nums: Vec<i32>) -> i32 {
        nums.iter().fold(0, |mut acc, &num| {
            acc |= num;
            acc
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let nums = vec![3, 2, 4, 6];
        assert_eq!(Solution::maximum_xor(nums), 7);

        let nums = vec![1, 2, 3, 9, 2];
        assert_eq!(Solution::maximum_xor(nums), 11);
    }
}
