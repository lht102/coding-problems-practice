struct Solution;

impl Solution {
    pub fn minimum_deletions(nums: Vec<i32>) -> i32 {
        let max = nums.iter().enumerate().map(|(i, &n)| (n, i)).max().unwrap();
        let min = nums.iter().enumerate().map(|(i, &n)| (n, i)).min().unwrap();
        let mut idx1 = max.1;
        let mut idx2 = min.1;
        if idx1 > idx2 {
            std::mem::swap(&mut idx1, &mut idx2);
        }
        let n = nums.len();
        (n - idx1).min(idx2 + 1).min(n - idx2 + idx1 + 1) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let nums = vec![2, 10, 7, 5, 4, 1, 8, 6];
        assert_eq!(Solution::minimum_deletions(nums), 5);

        let nums = vec![0, -4, 19, 1, 8, -2, -3, 5];
        assert_eq!(Solution::minimum_deletions(nums), 3);

        let nums = vec![101];
        assert_eq!(Solution::minimum_deletions(nums), 1);
    }
}
