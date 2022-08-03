use std::cmp::Reverse;
use std::collections::BinaryHeap;

struct Solution;

impl Solution {
    pub fn kth_smallest(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
        let n = matrix[0].len();
        let k = k as usize;
        let mut min_heap: BinaryHeap<Reverse<(i32, usize, usize)>> = BinaryHeap::new();
        for (r, arr) in matrix.iter().enumerate().take(k) {
            min_heap.push(Reverse((arr[0], r, 0)));
        }
        let mut res = -1;
        for _ in 0..k {
            if let Some(Reverse((val, r, c))) = min_heap.pop() {
                res = val;
                if c + 1 < n {
                    min_heap.push(Reverse((matrix[r][c + 1], r, c + 1)));
                }
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
        let matrix = vec![vec![1, 5, 9], vec![10, 11, 13], vec![12, 13, 15]];
        let k = 8;
        assert_eq!(Solution::kth_smallest(matrix, k), 13);

        let matrix = vec![vec![-5]];
        let k = 1;
        assert_eq!(Solution::kth_smallest(matrix, k), -5);
    }
}
