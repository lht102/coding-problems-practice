use std::{cmp::Reverse, collections::BinaryHeap, iter::FromIterator};

struct Solution;

impl Solution {
    pub fn k_smallest_pairs(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
        let mut pq = BinaryHeap::from_iter(
            nums1
                .iter()
                .enumerate()
                .map(|(i, &num)| Reverse((num + nums2[0], i, 0usize))),
        );
        let k = k as usize;
        let mut res = Vec::with_capacity(k);
        while let Some(Reverse((_, i, j))) = pq.pop() {
            res.push(vec![nums1[i], nums2[j]]);
            if res.len() >= k {
                break;
            }
            if j + 1 < nums2.len() {
                pq.push(Reverse((nums1[i] + nums2[j + 1], i, j + 1)));
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
        let nums1 = vec![1, 7, 11];
        let nums2 = vec![2, 4, 6];
        let k = 3;
        assert_eq!(
            Solution::k_smallest_pairs(nums1, nums2, k),
            vec![vec![1, 2], vec![1, 4], vec![1, 6]],
        );

        let nums1 = vec![1, 1, 2];
        let nums2 = vec![1, 2, 3];
        let k = 2;
        assert_eq!(
            Solution::k_smallest_pairs(nums1, nums2, k),
            vec![vec![1, 1], vec![1, 1]],
        );

        let nums1 = vec![1, 2];
        let nums2 = vec![3];
        let k = 3;
        assert_eq!(
            Solution::k_smallest_pairs(nums1, nums2, k),
            vec![vec![1, 3], vec![2, 3]],
        );
    }
}
