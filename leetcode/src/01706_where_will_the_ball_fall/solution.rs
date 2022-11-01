struct Solution;

impl Solution {
    pub fn find_ball(grid: Vec<Vec<i32>>) -> Vec<i32> {
        (0..grid[0].len())
            .map(|j| Solution::dfs(&grid, 0, j))
            .collect()
    }

    fn dfs(grid: &[Vec<i32>], si: usize, sj: usize) -> i32 {
        let (mut ci, mut cj) = (si, sj);
        while ci != grid.len() {
            if grid[ci][cj] == 1 {
                if cj + 1 >= grid[0].len() || grid[ci][cj + 1] != 1 {
                    return -1;
                }
                cj += 1;
            } else {
                if cj < 1 || grid[ci][cj - 1] != -1 {
                    return -1;
                }
                cj -= 1;
            }
            ci += 1;
        }
        cj as _
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let grid = vec![
            vec![1, 1, 1, -1, -1],
            vec![1, 1, 1, -1, -1],
            vec![-1, -1, -1, 1, 1],
            vec![1, 1, 1, 1, -1],
            vec![-1, -1, -1, -1, -1],
        ];
        assert_eq!(Solution::find_ball(grid), vec![1, -1, -1, -1, -1]);

        let grid = vec![vec![-1]];
        assert_eq!(Solution::find_ball(grid), vec![-1]);

        let grid = vec![
            vec![1, 1, 1, 1, 1, 1],
            vec![-1, -1, -1, -1, -1, -1],
            vec![1, 1, 1, 1, 1, 1],
            vec![-1, -1, -1, -1, -1, -1],
        ];
        assert_eq!(Solution::find_ball(grid), vec![0, 1, 2, 3, 4, -1]);
    }
}
