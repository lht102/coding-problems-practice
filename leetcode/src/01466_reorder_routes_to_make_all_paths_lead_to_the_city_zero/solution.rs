struct Solution;

impl Solution {
    pub fn min_reorder(n: i32, connections: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut adj = vec![vec![]; n];
        for conn in connections {
            let u = conn[0] as usize;
            let v = conn[1] as usize;
            adj[u].push((1, v));
            adj[v].push((0, u));
        }
        let mut res = 0;
        Self::dfs(&adj, 0, None, &mut res);
        res
    }

    fn dfs(adj: &[Vec<(i32, usize)>], source: usize, parent: Option<usize>, res: &mut i32) {
        for &(cost, destination) in &adj[source] {
            if parent == Some(destination) {
                continue;
            }
            *res += cost;
            Self::dfs(adj, destination, Some(source), res);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let n = 6;
        let connections = vec![vec![0, 1], vec![1, 3], vec![2, 3], vec![4, 0], vec![4, 5]];
        assert_eq!(Solution::min_reorder(n, connections), 3);

        let n = 5;
        let connections = vec![vec![1, 0], vec![1, 2], vec![3, 2], vec![3, 4]];
        assert_eq!(Solution::min_reorder(n, connections), 2);

        let n = 3;
        let connections = vec![vec![1, 0], vec![2, 0]];
        assert_eq!(Solution::min_reorder(n, connections), 0);
    }
}
