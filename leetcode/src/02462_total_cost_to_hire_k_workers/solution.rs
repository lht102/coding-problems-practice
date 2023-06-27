use std::{cmp::Reverse, collections::BinaryHeap};

struct Solution;

impl Solution {
    pub fn total_cost(costs: Vec<i32>, k: i32, candidates: i32) -> i64 {
        let k = k as usize;
        let candidates = candidates as usize;
        let (mut left_pq, mut right_pq) = (BinaryHeap::new(), BinaryHeap::new());
        let (mut left_idx, mut right_bound) = (0, costs.len());
        let mut res = 0i64;
        for _ in 0..k {
            while left_pq.len() < candidates && left_idx < right_bound {
                left_pq.push(Reverse(costs[left_idx]));
                left_idx += 1;
            }
            while right_pq.len() < candidates && left_idx < right_bound {
                right_pq.push(Reverse(costs[right_bound - 1]));
                right_bound -= 1;
            }
            let left_val = left_pq.peek().unwrap_or(&Reverse(i32::MAX)).0;
            let right_val = right_pq.peek().unwrap_or(&Reverse(i32::MAX)).0;
            if left_val <= right_val {
                res += left_val as i64;
                left_pq.pop();
            } else {
                res += right_val as i64;
                right_pq.pop();
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
        let costs = vec![17, 12, 10, 2, 7, 2, 11, 20, 8];
        let k = 3;
        let candidates = 4;
        assert_eq!(Solution::total_cost(costs, k, candidates), 11);

        let costs = vec![1, 2, 4, 1];
        let k = 3;
        let candidates = 3;
        assert_eq!(Solution::total_cost(costs, k, candidates), 4);

        let costs = vec![10, 1, 11, 10];
        let k = 2;
        let candidates = 1;
        assert_eq!(Solution::total_cost(costs, k, candidates), 11);
    }
}
