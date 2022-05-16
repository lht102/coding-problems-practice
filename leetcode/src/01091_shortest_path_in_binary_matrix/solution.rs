use std::collections::VecDeque;

struct Solution;

impl Solution {
    pub fn shortest_path_binary_matrix(grid: Vec<Vec<i32>>) -> i32 {
        let dirs: &'static [[i32; 2]; 8] = &[
            [0, 1],
            [0, -1],
            [1, 0],
            [-1, 0],
            [1, 1],
            [-1, 1],
            [1, -1],
            [-1, -1],
        ];
        if grid[0][0] == 1 {
            return -1;
        }
        let n = grid.len();
        if n == 1 {
            return 1;
        }
        let mut res = 1;
        let mut vi = vec![vec![false; n]; n];
        vi[0][0] = true;
        let mut q = VecDeque::new();
        q.push_back((0, 0));
        while !q.is_empty() {
            for _ in 0..q.len() {
                if let Some((i, j)) = q.pop_front() {
                    for &[di, dj] in dirs {
                        let (ni, nj) = (i + di, j + dj);
                        if ni < 0
                            || ni >= n as i32
                            || nj < 0
                            || nj >= n as i32
                            || grid[ni as usize][nj as usize] == 1
                            || vi[ni as usize][nj as usize]
                        {
                            continue;
                        }
                        if ni == n as i32 - 1 && nj == n as i32 - 1 {
                            return res + 1;
                        }
                        vi[ni as usize][nj as usize] = true;
                        q.push_back((ni, nj));
                    }
                }
            }
            res += 1;
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let grid = vec![vec![0, 1], vec![1, 0]];
        assert_eq!(Solution::shortest_path_binary_matrix(grid), 2);

        let grid = vec![vec![1, 0, 0], vec![1, 1, 0], vec![1, 1, 0]];
        assert_eq!(Solution::shortest_path_binary_matrix(grid), -1);

        let grid = vec![
            vec![0, 1, 1, 1, 1, 1, 1, 1],
            vec![0, 1, 1, 0, 0, 0, 0, 0],
            vec![0, 1, 0, 1, 1, 1, 1, 0],
            vec![0, 1, 0, 1, 1, 1, 1, 0],
            vec![0, 1, 1, 0, 0, 1, 1, 0],
            vec![0, 1, 1, 1, 1, 0, 1, 0],
            vec![0, 0, 0, 0, 0, 1, 1, 0],
            vec![1, 1, 1, 1, 1, 1, 1, 0],
        ];

        assert_eq!(Solution::shortest_path_binary_matrix(grid), 25);
    }
}
