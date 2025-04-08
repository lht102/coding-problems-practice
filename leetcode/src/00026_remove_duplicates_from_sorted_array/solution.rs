struct Solution;

impl Solution {
    #[allow(clippy::ptr_arg)]
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        let mut j = 1;
        for i in 1..nums.len() {
            if nums[i] != nums[j - 1] {
                nums[j] = nums[i];
                j += 1;
            }
        }
        j as _
    }
}
