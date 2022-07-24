use std::cmp::Ordering;

struct Solution;

impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let m = matrix.len() as i32;
        let n = matrix[0].len() as i32;
        let (mut i, mut j) = (0, n - 1);
        while i < m && j >= 0 {
            match matrix[i as usize][j as usize].cmp(&target) {
                Ordering::Less => i += 1,
                Ordering::Greater => j -= 1,
                Ordering::Equal => return true,
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let matrix = vec![
            vec![1, 4, 7, 11, 15],
            vec![2, 5, 8, 12, 19],
            vec![3, 6, 9, 16, 22],
            vec![10, 13, 14, 17, 24],
            vec![18, 21, 23, 26, 30],
        ];
        let target = 5;
        assert!(Solution::search_matrix(matrix, target));

        let matrix = vec![
            vec![1, 4, 7, 11, 15],
            vec![2, 5, 8, 12, 19],
            vec![3, 6, 9, 16, 22],
            vec![10, 13, 14, 17, 24],
            vec![18, 21, 23, 26, 30],
        ];
        let target = 20;
        assert!(!Solution::search_matrix(matrix, target));
    }
}
