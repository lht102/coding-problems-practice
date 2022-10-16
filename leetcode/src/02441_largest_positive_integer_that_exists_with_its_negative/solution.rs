use std::collections::HashSet;
use std::iter::FromIterator;

struct Solution;

impl Solution {
    pub fn find_max_k(nums: Vec<i32>) -> i32 {
        let set: HashSet<_> = HashSet::from_iter(nums.iter().cloned());
        nums.iter().fold(-1, |res, &num| {
            if set.contains(&num) && set.contains(&(-num)) {
                res.max(num)
            } else {
                res
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let nums = vec![-1, 2, -3, 3];
        assert_eq!(Solution::find_max_k(nums), 3);

        let nums = vec![-1, 10, 6, 7, -7, 1];
        assert_eq!(Solution::find_max_k(nums), 7);

        let nums = vec![-10, 8, 6, 7, -2, -3];
        assert_eq!(Solution::find_max_k(nums), -1);
    }
}
