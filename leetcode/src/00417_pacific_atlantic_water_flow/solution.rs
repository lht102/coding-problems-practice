use std::collections::HashSet;
use std::collections::VecDeque;
use std::iter::FromIterator;

struct Solution;

impl Solution {
    pub fn pacific_atlantic(heights: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let m = heights.len() as i32;
        let n = heights[0].len() as i32;
        let pacific = (0..m)
            .map(|i| (i, 0))
            .chain((0..n).map(|j| (0, j)))
            .collect::<Vec<_>>();
        let atlantic = (0..m)
            .map(|i| (i, n - 1))
            .chain((0..n).map(|j| (m - 1, j)))
            .collect::<Vec<_>>();
        Solution::bfs(&heights, m, n, &pacific)
            .intersection(&Solution::bfs(&heights, m, n, &atlantic))
            .map(|&x| vec![x.0, x.1])
            .collect()
    }

    fn bfs(heights: &[Vec<i32>], m: i32, n: i32, starts: &[(i32, i32)]) -> HashSet<(i32, i32)> {
        let dirs: &'static [i32] = &[0, 1, 0, -1, 0];
        let mut q = VecDeque::from_iter(starts.iter().cloned());
        let mut visited = HashSet::from_iter(starts.iter().cloned());
        while let Some((ci, cj)) = q.pop_back() {
            for i in 0..4 {
                let (ni, nj) = (ci + dirs[i], cj + dirs[i + 1]);
                if ni >= m
                    || ni < 0
                    || nj >= n
                    || nj < 0
                    || visited.contains(&(ni, nj))
                    || heights[ni as usize][nj as usize] < heights[ci as usize][cj as usize]
                {
                    continue;
                }
                visited.insert((ni, nj));
                q.push_back((ni, nj));
            }
        }
        visited
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let heights = vec![
            vec![1, 2, 2, 3, 5],
            vec![3, 2, 3, 4, 4],
            vec![2, 4, 5, 3, 1],
            vec![6, 7, 1, 4, 5],
            vec![5, 1, 1, 2, 4],
        ];
        assert_eq!(
            Solution::pacific_atlantic(heights)
                .into_iter()
                .collect::<HashSet<Vec<i32>>>(),
            vec![
                vec![0, 4],
                vec![1, 3],
                vec![1, 4],
                vec![2, 2],
                vec![3, 0],
                vec![3, 1],
                vec![4, 0]
            ]
            .into_iter()
            .collect()
        );

        let heights = vec![vec![1]];
        assert_eq!(Solution::pacific_atlantic(heights), vec![vec![0, 0]]);
    }
}
