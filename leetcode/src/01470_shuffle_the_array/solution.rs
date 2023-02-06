struct Solution;

impl Solution {
    pub fn shuffle(nums: Vec<i32>, n: i32) -> Vec<i32> {
        let n = n as usize;
        nums[..n]
            .iter()
            .zip(nums[n..].iter())
            .flat_map(|(&x, &y)| vec![x, y])
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let nums = vec![2, 5, 1, 3, 4, 7];
        let n = 3;
        assert_eq!(Solution::shuffle(nums, n), vec![2, 3, 5, 4, 1, 7]);

        let nums = vec![2, 5, 1, 3, 4, 7];
        let n = 3;
        assert_eq!(Solution::shuffle(nums, n), vec![2, 3, 5, 4, 1, 7]);
    }
}
