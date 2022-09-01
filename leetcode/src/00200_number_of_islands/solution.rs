struct Solution;

impl Solution {
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        let mut res = 0;
        let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == '1' && !visited[i][j] {
                    Solution::dfs(&grid, &mut visited, i as _, j as _);
                    res += 1;
                }
            }
        }
        res
    }

    fn dfs(grid: &[Vec<char>], visited: &mut [Vec<bool>], i: i32, j: i32) -> bool {
        if i >= grid.len() as _
            || i < 0
            || j >= grid[0].len() as _
            || j < 0
            || visited[i as usize][j as usize]
            || grid[i as usize][j as usize] != '1'
        {
            return false;
        }
        visited[i as usize][j as usize] = true;
        Solution::dfs(grid, visited, i + 1, j)
            || Solution::dfs(grid, visited, i - 1, j)
            || Solution::dfs(grid, visited, i, j + 1)
            || Solution::dfs(grid, visited, i, j - 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let grid = vec![
            vec!['1', '1', '1', '1', '0'],
            vec!['1', '1', '0', '1', '0'],
            vec!['1', '1', '0', '0', '0'],
            vec!['0', '0', '0', '0', '0'],
        ];
        assert_eq!(Solution::num_islands(grid), 1);

        let grid = vec![
            vec!['1', '1', '0', '0', '0'],
            vec!['1', '1', '0', '0', '0'],
            vec!['0', '0', '1', '0', '0'],
            vec!['0', '0', '0', '1', '1'],
        ];
        assert_eq!(Solution::num_islands(grid), 3);
    }
}
