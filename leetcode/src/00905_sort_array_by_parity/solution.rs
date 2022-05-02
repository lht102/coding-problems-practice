struct Solution;

impl Solution {
    pub fn sort_array_by_parity(nums: Vec<i32>) -> Vec<i32> {
        nums.iter()
            .filter(|&&n| n % 2 == 0)
            .chain(nums.iter().filter(|&&n| n % 2 == 1))
            .cloned()
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let nums = vec![3, 1, 2, 4];
        assert_eq!(Solution::sort_array_by_parity(nums), vec![2, 4, 3, 1]);

        let nums = vec![0];
        assert_eq!(Solution::sort_array_by_parity(nums), vec![0]);
    }
}
