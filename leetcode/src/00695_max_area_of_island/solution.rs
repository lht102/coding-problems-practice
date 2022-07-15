struct Solution;

impl Solution {
    pub fn max_area_of_island(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let mut vi = vec![vec![false; n]; m];
        let mut res = 0;
        for (i, v) in grid.iter().enumerate() {
            for (j, &num) in v.iter().enumerate() {
                if !vi[i][j] && num == 1 {
                    res = res.max(Solution::dfs(&grid, &mut vi, i as i32, j as i32));
                }
            }
        }
        res
    }

    fn dfs(g: &[Vec<i32>], vi: &mut [Vec<bool>], i: i32, j: i32) -> i32 {
        if i < 0
            || i >= g.len() as i32
            || j < 0
            || j >= g[0].len() as i32
            || vi[i as usize][j as usize]
            || g[i as usize][j as usize] == 0
        {
            return 0;
        }
        vi[i as usize][j as usize] = true;
        1 + Solution::dfs(g, vi, i - 1, j)
            + Solution::dfs(g, vi, i + 1, j)
            + Solution::dfs(g, vi, i, j - 1)
            + Solution::dfs(g, vi, i, j + 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let grid = vec![
            vec![0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0],
            vec![0, 1, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 1, 0, 0],
            vec![0, 1, 0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0],
        ];
        assert_eq!(Solution::max_area_of_island(grid), 6);

        let grid = vec![vec![0, 0, 0, 0, 0, 0, 0, 0]];
        assert_eq!(Solution::max_area_of_island(grid), 0);
    }
}
