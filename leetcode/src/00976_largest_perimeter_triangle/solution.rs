use std::cmp::Reverse;

struct Solution;

impl Solution {
    pub fn largest_perimeter(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort_by_key(|k| Reverse(*k));
        nums.windows(3)
            .find_map(|arr| (arr[0] < arr[1] + arr[2]).then_some(arr.iter().sum()))
            .unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let nums = vec![2, 1, 2];
        assert_eq!(Solution::largest_perimeter(nums), 5);

        let nums = vec![1, 2, 1];
        assert_eq!(Solution::largest_perimeter(nums), 0);
    }
}
