use std::collections::BTreeSet;

struct Solution;

impl Solution {
    pub fn max_sum_submatrix(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
        let m = matrix.len();
        let n = matrix[0].len();
        let mut res = i32::MIN;
        for start_row in 0..m {
            let mut arr = vec![0; n];
            for end_row_arr in matrix.iter().take(m).skip(start_row) {
                let mut best_sum = 0;
                let mut cur_sum = 0;
                for (col, item) in arr.iter_mut().enumerate().take(n) {
                    *item += end_row_arr[col];
                    cur_sum = (*item).max(cur_sum + *item);
                    best_sum = best_sum.max(cur_sum);
                }
                if best_sum <= k {
                    res = res.max(best_sum);
                    continue;
                }
                res = res.max(Solution::max_subarray_sum(&arr, k));
            }
        }
        res
    }

    fn max_subarray_sum(arr: &[i32], k: i32) -> i32 {
        let mut res = i32::MIN;
        let mut set = BTreeSet::from([0]);
        let mut cur_sum = 0;
        for num in arr {
            cur_sum += num;
            if let Some(&prefix_sum) = set.range(cur_sum - k..).next() {
                res = res.max(cur_sum - prefix_sum);
            }
            set.insert(cur_sum);
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let matrix = vec![vec![1, 0, 1], vec![0, -2, -3]];
        let k = 2;
        assert_eq!(Solution::max_sum_submatrix(matrix, k), 2);

        let matrix = vec![vec![2, 2, -1]];
        let k = 3;
        assert_eq!(Solution::max_sum_submatrix(matrix, k), 3);
    }
}
