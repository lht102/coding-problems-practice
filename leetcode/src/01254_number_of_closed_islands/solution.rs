struct Solution;

impl Solution {
    pub fn closed_island(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let mut visited = vec![vec![false; n]; m];
        for i in 0..m {
            Self::dfs(&grid, &mut visited, i, 0);
            Self::dfs(&grid, &mut visited, i, n - 1);
        }
        for j in 0..n {
            Self::dfs(&grid, &mut visited, 0, j);
            Self::dfs(&grid, &mut visited, m - 1, j);
        }
        let mut res = 0;
        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == 0 && !visited[i][j] {
                    Self::dfs(&grid, &mut visited, i, j);
                    res += 1;
                }
            }
        }
        res
    }

    fn dfs(grid: &[Vec<i32>], visited: &mut [Vec<bool>], ci: usize, cj: usize) {
        if visited[ci][cj] || grid[ci][cj] == 1 {
            return;
        }
        visited[ci][cj] = true;
        if ci >= 1 {
            Self::dfs(grid, visited, ci - 1, cj);
        }
        if ci + 1 < grid.len() {
            Self::dfs(grid, visited, ci + 1, cj);
        }
        if cj >= 1 {
            Self::dfs(grid, visited, ci, cj - 1);
        }
        if cj + 1 < grid[ci].len() {
            Self::dfs(grid, visited, ci, cj + 1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let grid = vec![
            vec![1, 1, 1, 1, 1, 1, 1, 0],
            vec![1, 0, 0, 0, 0, 1, 1, 0],
            vec![1, 0, 1, 0, 1, 1, 1, 0],
            vec![1, 0, 0, 0, 0, 1, 0, 1],
            vec![1, 1, 1, 1, 1, 1, 1, 0],
        ];
        assert_eq!(Solution::closed_island(grid), 2);

        let grid = vec![
            vec![0, 0, 1, 0, 0],
            vec![0, 1, 0, 1, 0],
            vec![0, 1, 1, 1, 0],
        ];
        assert_eq!(Solution::closed_island(grid), 1);

        let grid = vec![
            vec![1, 1, 1, 1, 1, 1, 1],
            vec![1, 0, 0, 0, 0, 0, 1],
            vec![1, 0, 1, 1, 1, 0, 1],
            vec![1, 0, 1, 0, 1, 0, 1],
            vec![1, 0, 1, 1, 1, 0, 1],
            vec![1, 0, 0, 0, 0, 0, 1],
            vec![1, 1, 1, 1, 1, 1, 1],
        ];
        assert_eq!(Solution::closed_island(grid), 2);
    }
}
