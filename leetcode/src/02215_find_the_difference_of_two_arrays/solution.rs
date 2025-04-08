use std::collections::BTreeSet;

struct Solution;

impl Solution {
    pub fn find_difference(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<Vec<i32>> {
        let s1 = BTreeSet::from_iter(nums1);
        let s2 = BTreeSet::from_iter(nums2);
        vec![
            s1.difference(&s2).copied().collect(),
            s2.difference(&s1).copied().collect(),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let nums1 = vec![1, 2, 3];
        let nums2 = vec![2, 4, 6];
        assert_eq!(
            Solution::find_difference(nums1, nums2),
            vec![vec![1, 3], vec![4, 6]]
        );

        let nums1 = vec![1, 2, 3, 3];
        let nums2 = vec![1, 1, 2, 2];
        assert_eq!(
            Solution::find_difference(nums1, nums2),
            vec![vec![3], vec![]]
        );
    }
}
