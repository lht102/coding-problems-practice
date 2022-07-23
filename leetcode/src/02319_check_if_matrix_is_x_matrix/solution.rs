struct Solution;

impl Solution {
    pub fn check_x_matrix(grid: Vec<Vec<i32>>) -> bool {
        for (i, v) in grid.iter().enumerate() {
            for (j, &num) in v.iter().enumerate() {
                if Solution::is_diagonal(i, j, grid.len()) {
                    if num == 0 {
                        return false;
                    }
                } else if num != 0 {
                    return false;
                }
            }
        }
        true
    }

    fn is_diagonal(i: usize, j: usize, n: usize) -> bool {
        i == j || i + j + 1 == n
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let grid = vec![
            vec![2, 0, 0, 1],
            vec![0, 3, 1, 0],
            vec![0, 5, 2, 0],
            vec![4, 0, 0, 2],
        ];
        assert!(Solution::check_x_matrix(grid));

        let grid = vec![vec![5, 7, 0], vec![0, 3, 1], vec![0, 5, 0]];
        assert!(!Solution::check_x_matrix(grid));
    }
}
