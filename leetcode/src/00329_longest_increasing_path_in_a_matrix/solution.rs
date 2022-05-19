struct Solution;

impl Solution {
    pub fn longest_increasing_path(matrix: Vec<Vec<i32>>) -> i32 {
        let mut dp = vec![vec![0; matrix[0].len()]; matrix.len()];
        let mut res = 0;
        for i in 0..matrix.len() {
            for j in 0..matrix[0].len() {
                res = res.max(Solution::dfs(&matrix, i as i32, j as i32, &mut dp));
            }
        }
        res
    }

    fn dfs(matrix: &Vec<Vec<i32>>, r: i32, c: i32, dp: &mut Vec<Vec<i32>>) -> i32 {
        let dirs: &'static [i32] = &[0, 1, 0, -1, 0];
        if dp[r as usize][c as usize] != 0 {
            return dp[r as usize][c as usize];
        }
        let mut res = 1;
        for i in 0..4 {
            let (nr, nc) = (r + dirs[i], c + dirs[i + 1]);
            if nr < 0
                || nr >= matrix.len() as i32
                || nc < 0
                || nc >= matrix[0].len() as i32
                || matrix[nr as usize][nc as usize] >= matrix[r as usize][c as usize]
            {
                continue;
            }
            res = res.max(Solution::dfs(matrix, nr, nc, dp) + 1);
        }
        dp[r as usize][c as usize] = res;
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let matrix = vec![vec![3, 4, 5], vec![3, 2, 6], vec![2, 2, 1]];
        assert_eq!(Solution::longest_increasing_path(matrix), 4);

        let matrix = vec![vec![1]];
        assert_eq!(Solution::longest_increasing_path(matrix), 1);
    }
}
