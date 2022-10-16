use std::collections::HashSet;
use std::iter::FromIterator;

struct Solution;

impl Solution {
    pub fn count_distinct_integers(nums: Vec<i32>) -> i32 {
        HashSet::<i32>::from_iter(
            nums.iter()
                .cloned()
                .chain(nums.iter().cloned().map(Solution::reverse_i32)),
        )
        .len() as _
    }

    fn reverse_i32(n: i32) -> i32 {
        let mut res = 0;
        let mut tmp = n;
        while tmp != 0 {
            res = res * 10 + tmp % 10;
            tmp /= 10;
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let nums = vec![1, 13, 10, 12, 31];
        assert_eq!(Solution::count_distinct_integers(nums), 6);

        let nums = vec![2, 2, 2];
        assert_eq!(Solution::count_distinct_integers(nums), 1);
    }
}
