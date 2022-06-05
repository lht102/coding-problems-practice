use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn array_change(nums: Vec<i32>, operations: Vec<Vec<i32>>) -> Vec<i32> {
        let mut nums = nums;
        let mut num_to_idx =
            nums.iter()
                .enumerate()
                .fold(HashMap::<i32, usize>::new(), |mut map, (i, &num)| {
                    map.insert(num, i);
                    map
                });
        for op in &operations {
            let num = op[0];
            let new_num = op[1];
            if let Some(&idx) = num_to_idx.get(&num) {
                nums[idx] = new_num;
                num_to_idx.remove(&num);
                num_to_idx.insert(new_num, idx);
            }
        }
        nums
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let nums = vec![1, 2, 4, 6];
        let operations = vec![vec![1, 3], vec![4, 7], vec![6, 1]];
        assert_eq!(Solution::array_change(nums, operations), vec![3, 2, 7, 1]);

        let nums = vec![1, 2];
        let operations = vec![vec![1, 3], vec![2, 1], vec![3, 2]];
        assert_eq!(Solution::array_change(nums, operations), vec![2, 1]);
    }
}
