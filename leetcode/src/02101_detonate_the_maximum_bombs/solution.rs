struct Solution;

impl Solution {
    pub fn maximum_detonation(bombs: Vec<Vec<i32>>) -> i32 {
        let n = bombs.len();
        let mut adj = vec![vec![]; n];
        for (i, b1) in bombs.iter().enumerate() {
            for (j, b2) in bombs.iter().enumerate() {
                if i == j {
                    continue;
                }
                if Self::can_detnoate(b1[0], b1[1], b1[2], b2[0], b2[1]) {
                    adj[i].push(j);
                }
            }
        }
        (0..n)
            .map(|i| {
                let mut visited = vec![false; n];
                Self::dfs(&adj, i, &mut visited)
            })
            .max()
            .unwrap()
    }

    fn dfs(adj: &[Vec<usize>], i: usize, visited: &mut [bool]) -> i32 {
        if visited[i] {
            return 0;
        }
        visited[i] = true;
        let mut res = 1;
        for &j in &adj[i] {
            res += Self::dfs(adj, j, visited);
        }
        res
    }

    fn can_detnoate(x1: i32, y1: i32, r1: i32, x2: i32, y2: i32) -> bool {
        let diff_x = (x2 - x1) as i64;
        let diff_y = (y2 - y1) as i64;
        let r = r1 as i64;
        diff_x * diff_x + diff_y * diff_y <= r * r
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let bombs = vec![vec![2, 1, 3], vec![6, 1, 4]];
        assert_eq!(Solution::maximum_detonation(bombs), 2);

        let bombs = vec![vec![1, 1, 5], vec![10, 10, 5]];
        assert_eq!(Solution::maximum_detonation(bombs), 1);

        let bombs = vec![
            vec![1, 2, 3],
            vec![2, 3, 1],
            vec![3, 4, 2],
            vec![4, 5, 3],
            vec![5, 6, 4],
        ];
        assert_eq!(Solution::maximum_detonation(bombs), 5);

        let bombs = vec![vec![1, 1, 100000], vec![100000, 100000, 1]];
        assert_eq!(Solution::maximum_detonation(bombs), 1);
    }
}
