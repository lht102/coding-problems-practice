struct Solution;

impl Solution {
    pub fn increasing_triplet(nums: Vec<i32>) -> bool {
        let right_arr = nums
            .iter()
            .rev()
            .scan(*nums.last().unwrap(), |maximum, &num| {
                if num > *maximum {
                    *maximum = num;
                }
                Some(*maximum)
            })
            .collect::<Vec<_>>();

        let left_arr = nums
            .iter()
            .scan(*nums.first().unwrap(), |minimum, &num| {
                if num < *minimum {
                    *minimum = num;
                }
                Some(*minimum)
            })
            .collect::<Vec<_>>();
        (1..nums.len() - 1)
            .any(|i| left_arr[i - 1] < nums[i] && nums[i] < right_arr[right_arr.len() - (i + 1)])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let nums = vec![1, 2, 3, 4, 5];
        assert!(Solution::increasing_triplet(nums));

        let nums = vec![5, 4, 3, 2, 1];
        assert!(!Solution::increasing_triplet(nums));

        let nums = vec![2, 1, 5, 0, 4, 6];
        assert!(Solution::increasing_triplet(nums));

        let nums = vec![20, 100, 10, 12, 5, 13];
        assert!(Solution::increasing_triplet(nums));

        let nums = vec![0, 4, 2, 1, 0, -1, -3];
        assert!(!Solution::increasing_triplet(nums));
    }
}
