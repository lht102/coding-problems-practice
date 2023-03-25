struct Solution;

impl Solution {
    pub fn make_connected(n: i32, connections: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        if connections.len() + 1 < n {
            return -1;
        }

        let mut adj = vec![vec![]; n];
        for conn in connections {
            let u = conn[0] as usize;
            let v = conn[1] as usize;
            adj[u].push(v);
            adj[v].push(u);
        }

        let mut res = 0;
        let mut visited = vec![false; n];
        for u in 0..n {
            if !visited[u] {
                Self::dfs(&adj, &mut visited, u);
                res += 1;
            }
        }
        res - 1
    }

    fn dfs(adj: &[Vec<usize>], visited: &mut [bool], cur: usize) {
        visited[cur] = true;
        for &next in &adj[cur] {
            if visited[next] {
                continue;
            }
            Self::dfs(adj, visited, next);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let n = 4;
        let connections = vec![vec![0, 1], vec![0, 2], vec![1, 2]];
        assert_eq!(Solution::make_connected(n, connections), 1);

        let n = 6;
        let connections = vec![vec![0, 1], vec![0, 2], vec![0, 3], vec![1, 2]];
        assert_eq!(Solution::make_connected(n, connections), -1);
    }
}
