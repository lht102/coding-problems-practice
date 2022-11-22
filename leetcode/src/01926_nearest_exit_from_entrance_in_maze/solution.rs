use std::collections::VecDeque;

struct Solution;

impl Solution {
    pub fn nearest_exit(maze: Vec<Vec<char>>, entrance: Vec<i32>) -> i32 {
        let m = maze.len() as i32;
        let n = maze[0].len() as i32;
        let mut visited = vec![vec![false; n as usize]; m as usize];
        let mut q = VecDeque::from([(entrance[0], entrance[1])]);
        visited[entrance[0] as usize][entrance[1] as usize] = true;
        let dirs: &'static [i32] = &[0, 1, 0, -1, 0];
        let mut res = 0;
        while !q.is_empty() {
            for _ in 0..q.len() {
                if let Some((ci, cj)) = q.pop_front() {
                    if (ci == 0 || ci == m - 1 || cj == 0 || cj == n - 1)
                        && maze[ci as usize][cj as usize] == '.'
                        && (ci != entrance[0] || cj != entrance[1])
                    {
                        return res;
                    }
                    for i in 0..4 {
                        let ni = ci + dirs[i];
                        let nj = cj + dirs[i + 1];
                        if ni < 0
                            || ni >= m
                            || nj < 0
                            || nj >= n
                            || visited[ni as usize][nj as usize]
                            || maze[ni as usize][nj as usize] == '+'
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
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let maze = vec![
            vec!['+', '+', '.', '+'],
            vec!['.', '.', '.', '+'],
            vec!['+', '+', '+', '.'],
        ];
        let entrance = vec![1, 2];
        assert_eq!(Solution::nearest_exit(maze, entrance), 1);

        let maze = vec![
            vec!['+', '+', '+'],
            vec!['.', '.', '.'],
            vec!['+', '+', '+'],
        ];
        let entrance = vec![1, 0];
        assert_eq!(Solution::nearest_exit(maze, entrance), 2);

        let maze = vec![vec!['.', '+']];
        let entrance = vec![0, 0];
        assert_eq!(Solution::nearest_exit(maze, entrance), -1);
    }
}
