struct Solution;

impl Solution {
    pub fn array_sign(nums: Vec<i32>) -> i32 {
        *nums.iter().find(|&&x| x == 0).unwrap_or_else(|| {
            if nums.iter().filter(|&&x| x < 0).count() % 2 == 0 {
                &1
            } else {
                &-1
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let nums = vec![-1, -2, -3, -4, 3, 2, 1];
        assert_eq!(Solution::array_sign(nums), 1);

        let nums = vec![1, 5, 0, 2, -3];
        assert_eq!(Solution::array_sign(nums), 0);

        let nums = vec![-1, 1, -1, 1, -1];
        assert_eq!(Solution::array_sign(nums), -1);
    }
}
