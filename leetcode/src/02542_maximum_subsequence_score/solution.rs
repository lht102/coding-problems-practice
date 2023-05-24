use std::cmp::Reverse;
use std::collections::BinaryHeap;

struct Solution;

impl Solution {
    pub fn max_score(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> i64 {
        let k = k as usize;
        let pairs = {
            let mut v = nums1
                .into_iter()
                .map(|x| x as i64)
                .zip(nums2.into_iter().map(|x| x as i64))
                .collect::<Vec<_>>();
            v.sort_unstable_by(|a, b| b.1.cmp(&a.1));
            v
        };
        let mut pq = BinaryHeap::new();
        let mut res = 0;
        let mut sum = 0;
        for (n1, n2) in pairs {
            sum += n1;
            pq.push(Reverse(n1));
            if pq.len() > k {
                sum -= pq.pop().unwrap().0;
            }
            if pq.len() == k {
                res = res.max(sum * n2);
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
        let nums1 = vec![1, 3, 3, 2];
        let nums2 = vec![2, 1, 3, 4];
        let k = 3;
        assert_eq!(Solution::max_score(nums1, nums2, k), 12);

        let nums1 = vec![4, 2, 3, 1, 1];
        let nums2 = vec![7, 5, 10, 9, 6];
        let k = 1;
        assert_eq!(Solution::max_score(nums1, nums2, k), 30);
    }
}
