struct Solution;

impl Solution {
    pub fn find_max_fish(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let mut visited = vec![vec![false; n]; m];
        let mut res = 0;
        for i in 0..m {
            for j in 0..n {
                if !visited[i][j] {
                    res = res.max(Self::dfs(&grid, &mut visited, i, j));
                }
            }
        }
        res
    }

    fn dfs(g: &[Vec<i32>], visited: &mut [Vec<bool>], ci: usize, cj: usize) -> i32 {
        if visited[ci][cj] || g[ci][cj] == 0 {
            return 0;
        }
        visited[ci][cj] = true;
        let mut res = g[ci][cj];
        if ci >= 1 {
            res += Self::dfs(g, visited, ci - 1, cj);
        }
        if ci + 1 < g.len() {
            res += Self::dfs(g, visited, ci + 1, cj);
        }
        if cj >= 1 {
            res += Self::dfs(g, visited, ci, cj - 1);
        }
        if cj + 1 < g[ci].len() {
            res += Self::dfs(g, visited, ci, cj + 1);
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let grid = vec![
            vec![0, 2, 1, 0],
            vec![4, 0, 0, 3],
            vec![1, 0, 0, 4],
            vec![0, 3, 2, 0],
        ];
        assert_eq!(Solution::find_max_fish(grid), 7);

        let grid = vec![
            vec![1, 0, 0, 0],
            vec![0, 0, 0, 0],
            vec![0, 0, 0, 0],
            vec![0, 0, 0, 1],
        ];
        assert_eq!(Solution::find_max_fish(grid), 1);
    }
}
