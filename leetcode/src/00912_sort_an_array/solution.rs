struct Solution;

impl Solution {
    pub fn sort_array(nums: Vec<i32>) -> Vec<i32> {
        Self::merge_sort(&nums)
    }

    fn merge_sort(nums: &[i32]) -> Vec<i32> {
        match nums.len() {
            0 | 1 => nums.to_vec(),
            n => {
                let mid = n / 2;
                let arr1 = Self::merge_sort(&nums[..mid]);
                let arr2 = Self::merge_sort(&nums[mid..]);
                Self::merge(&arr1, &arr2)
            }
        }
    }

    fn merge(arr1: &[i32], arr2: &[i32]) -> Vec<i32> {
        let mut res = vec![0; arr1.len() + arr2.len()];
        let (mut i, mut j) = (0, 0);
        for num in &mut res {
            if i < arr1.len() && (j >= arr2.len() || arr1[i] <= arr2[j]) {
                *num = arr1[i];
                i += 1;
            } else {
                *num = arr2[j];
                j += 1;
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let nums = vec![5, 2, 3, 1];
        assert_eq!(Solution::sort_array(nums), vec![1, 2, 3, 5]);

        let nums = vec![5, 1, 1, 2, 0, 0];
        assert_eq!(Solution::sort_array(nums), vec![0, 0, 1, 1, 2, 5]);
    }
}
