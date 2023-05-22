use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::iter::FromIterator;

struct Solution;

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut pq = BinaryHeap::from_iter(
            nums.iter()
                .fold(HashMap::<i32, usize>::new(), |mut map, &x| {
                    *map.entry(x).or_default() += 1;
                    map
                })
                .into_iter()
                .map(|(k, v)| (v, k)),
        );
        (0..k).map(|_| pq.pop().unwrap().1).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let nums = vec![1, 1, 1, 2, 2, 3];
        let k = 2;
        assert_eq!(Solution::top_k_frequent(nums, k), vec![1, 2]);

        let nums = vec![1];
        let k = 1;
        assert_eq!(Solution::top_k_frequent(nums, k), vec![1]);
    }
}
