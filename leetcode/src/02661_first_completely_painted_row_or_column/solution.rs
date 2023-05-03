use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn first_complete_index(arr: Vec<i32>, mat: Vec<Vec<i32>>) -> i32 {
        let m = mat.len();
        let n = mat[0].len();
        let mut pos_by_val = HashMap::new();
        for (i, row) in mat.iter().enumerate() {
            for (j, &num) in row.iter().enumerate() {
                pos_by_val.insert(num, (i, j));
            }
        }
        let mut colored_cnt_col = vec![0; n];
        let mut colored_cnt_row = vec![0; m];
        for (idx, x) in arr.iter().enumerate() {
            if let Some(&(i, j)) = pos_by_val.get(x) {
                colored_cnt_row[i] += 1;
                colored_cnt_col[j] += 1;
                if colored_cnt_row[i] == n || colored_cnt_col[j] == m {
                    return idx as _;
                }
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let arr = vec![1, 3, 4, 2];
        let mat = vec![vec![1, 4], vec![2, 3]];
        assert_eq!(Solution::first_complete_index(arr, mat), 2);

        let arr = vec![2, 8, 7, 4, 1, 3, 5, 6, 9];
        let mat = vec![vec![3, 2, 5], vec![1, 4, 6], vec![8, 7, 9]];
        assert_eq!(Solution::first_complete_index(arr, mat), 3);
    }
}
