struct Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let (mut lo, mut hi) = (0, nums.len() - 1);
        while lo < hi {
            let mid = (hi - lo) / 2 + lo;
            if nums[mid] < target {
                lo = mid + 1;
            } else {
                hi = mid;
            }
        }
        if nums[lo] == target {
            lo as i32
        } else {
            -1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let nums = vec![-1, 0, 3, 5, 9, 12];
        let target = 9;
        assert_eq!(Solution::search(nums, target), 4);

        let nums = vec![-1, 0, 3, 5, 9, 12];
        let target = 2;
        assert_eq!(Solution::search(nums, target), -1);
    }
}
