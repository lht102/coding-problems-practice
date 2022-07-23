use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn num_submatrix_sum_target(matrix: Vec<Vec<i32>>, target: i32) -> i32 {
        let _m = matrix.len();
        let n = matrix[0].len();
        let mut matrix = matrix;
        for arr in matrix.iter_mut() {
            for j in 1..n {
                arr[j] += arr[j - 1];
            }
        }
        let mut res = 0;
        for from_col in 0..n {
            for to_col in from_col..n {
                let mut total = 0;
                let mut freq = HashMap::from([(0, 1)]);
                for arr in matrix.iter() {
                    total += arr[to_col] - if from_col == 0 { 0 } else { arr[from_col - 1] };
                    if let Some(cnt) = freq.get(&(total - target)) {
                        res += cnt;
                    }
                    *freq.entry(total).or_default() += 1;
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
        let matrix = vec![vec![0, 1, 0], vec![1, 1, 1], vec![0, 1, 0]];
        let target = 0;
        assert_eq!(Solution::num_submatrix_sum_target(matrix, target), 4);

        let matrix = vec![vec![1, -1], vec![-1, 1]];
        let target = 0;
        assert_eq!(Solution::num_submatrix_sum_target(matrix, target), 5);

        let matrix = vec![vec![904]];
        let target = 0;
        assert_eq!(Solution::num_submatrix_sum_target(matrix, target), 0);
    }
}
