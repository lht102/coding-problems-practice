use std::collections::VecDeque;

struct Solution;

impl Solution {
    pub fn max_distance(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let mut q = VecDeque::new();
        let mut visited = vec![vec![false; n]; n];
        for i in 0..n {
            for j in 0..n {
                if grid[i][j] == 1 {
                    visited[i][j] = true;
                    q.push_back((i as i32, j as i32));
                }
            }
        }
        let n = n as i32;
        let mut res = -1;
        while !q.is_empty() {
            for _ in 0..q.len() {
                if let Some((ci, cj)) = q.pop_front() {
                    let dirs: &'static [i32] = &[0, 1, 0, -1, 0];
                    for i in 0..4 {
                        let (ni, nj) = (ci + dirs[i], cj + dirs[i + 1]);
                        if ni < 0
                            || ni >= n
                            || nj < 0
                            || nj >= n
                            || visited[ni as usize][nj as usize]
                        {
                            continue;
                        }
                        visited[ni as usize][nj as usize] = true;
                        q.push_back((ni, nj));
                    }
                }
            }
            res += 1;
        }
        if res == 0 {
            -1
        } else {
            res
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let grid = vec![vec![1, 0, 1], vec![0, 0, 0], vec![1, 0, 1]];
        assert_eq!(Solution::max_distance(grid), 2);

        let grid = vec![vec![1, 0, 0], vec![0, 0, 0], vec![0, 0, 0]];
        assert_eq!(Solution::max_distance(grid), 4);
    }
}
