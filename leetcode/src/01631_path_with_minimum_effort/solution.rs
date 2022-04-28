struct Solution;

impl Solution {
    pub fn minimum_effort_path(heights: Vec<Vec<i32>>) -> i32 {
        let m = heights.len();
        let n = heights[0].len();

        let (mut lo, mut hi) = (0, 1000000);
        while lo < hi {
            let mid = (hi - lo) / 2 + lo;
            let mut vi = vec![vec![false; n]; m];
            if Self::dfs(&heights, &mut vi, 0, 0, mid) {
                hi = mid;
            } else {
                lo = mid + 1;
            }
        }
        lo
    }

    fn dfs(g: &Vec<Vec<i32>>, vi: &mut Vec<Vec<bool>>, ci: i32, cj: i32, k: i32) -> bool {
        if ci as usize == g.len() - 1 && cj as usize == g[0].len() - 1 {
            return true;
        }
        vi[ci as usize][cj as usize] = true;
        let dirs = [-1, 0, 1, 0, -1];
        for i in 0..4 {
            let ni = ci + dirs[i];
            let nj = cj + dirs[i + 1];
            if ni < 0
                || ni as usize >= g.len()
                || nj < 0
                || nj as usize >= g[0].len()
                || vi[ni as usize][nj as usize]
            {
                continue;
            }
            if (g[ni as usize][nj as usize] - g[ci as usize][cj as usize]).abs() <= k
                && Self::dfs(g, vi, ci + dirs[i], cj + dirs[i + 1], k)
            {
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let heights = vec![vec![1, 2, 2], vec![3, 8, 2], vec![5, 3, 5]];
        assert_eq!(Solution::minimum_effort_path(heights), 2);

        let heights = vec![vec![1, 2, 3], vec![3, 8, 4], vec![5, 3, 5]];
        assert_eq!(Solution::minimum_effort_path(heights), 1);

        let heights = vec![
            vec![1, 2, 1, 1, 1],
            vec![1, 2, 1, 2, 1],
            vec![1, 2, 1, 2, 1],
            vec![1, 2, 1, 2, 1],
            vec![1, 1, 1, 2, 1],
        ];
        assert_eq!(Solution::minimum_effort_path(heights), 0);
    }
}
