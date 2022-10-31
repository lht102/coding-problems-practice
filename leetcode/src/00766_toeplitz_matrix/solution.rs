use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn is_toeplitz_matrix(matrix: Vec<Vec<i32>>) -> bool {
        let mut group = HashMap::new();
        for i in 0..matrix.len() {
            for j in 0..matrix[0].len() {
                let key = i as i32 - j as i32;
                if let Some(&val) = group.get(&key) {
                    if val != matrix[i][j] {
                        return false;
                    }
                } else {
                    group.insert(key, matrix[i][j]);
                }
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let matrix = vec![vec![1, 2, 3, 4], vec![5, 1, 2, 3], vec![9, 5, 1, 2]];
        assert!(Solution::is_toeplitz_matrix(matrix));

        let matrix = vec![vec![1, 2], vec![2, 2]];
        assert!(!Solution::is_toeplitz_matrix(matrix));
    }
}
