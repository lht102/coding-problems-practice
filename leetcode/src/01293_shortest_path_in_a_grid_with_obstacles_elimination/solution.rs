use std::collections::VecDeque;

struct Solution;

impl Solution {
    pub fn shortest_path(grid: Vec<Vec<i32>>, k: i32) -> i32 {
        let m = grid.len() as i32;
        let n = grid[0].len() as i32;
        let mut q = VecDeque::from([(0, 0, k, 0)]);
        let mut visited = vec![vec![vec![false; k as usize + 1]; n as usize]; m as usize];
        visited[0][0][k as usize] = true;
        let dirs: &'static [i32] = &[0, 1, 0, -1, 0];
        while let Some((ci, cj, ck, cd)) = q.pop_front() {
            if ci == m - 1 && cj == n - 1 {
                return cd;
            }
            for i in 0..4 {
                let ni = ci + dirs[i];
                let nj = cj + dirs[i + 1];
                if ni < 0 || ni >= m || nj < 0 || nj >= n {
                    continue;
                }
                let ni = ni as usize;
                let nj = nj as usize;
                let nk = ck - grid[ni][nj];
                if nk < 0 {
                    continue;
                }
                let nk = nk as usize;
                if visited[ni][nj][nk] {
                    continue;
                }
                visited[ni][nj][nk] = true;
                q.push_back((ni as _, nj as _, nk as _, cd + 1));
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
        let grid = vec![
            vec![0, 0, 0],
            vec![1, 1, 0],
            vec![0, 0, 0],
            vec![0, 1, 1],
            vec![0, 0, 0],
        ];
        let k = 1;
        assert_eq!(Solution::shortest_path(grid, k), 6);

        let grid = vec![vec![0, 1, 1], vec![1, 1, 1], vec![1, 0, 0]];
        let k = 1;
        assert_eq!(Solution::shortest_path(grid, k), -1);
    }
}
