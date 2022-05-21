struct Solution;

impl Solution {
    pub fn critical_connections(n: i32, connections: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = n as usize;
        let mut adj: Vec<Vec<usize>> = vec![vec![]; n];
        for c in &connections {
            let u = c[0] as usize;
            let v = c[1] as usize;
            adj[u].push(v);
            adj[v].push(u);
        }

        let mut visited = vec![false; n];
        let mut t = 0;
        let mut tin = vec![0; n];
        let mut low = vec![0; n];
        let mut res = Vec::new();
        for i in 0..n {
            if !visited[i] {
                Solution::dfs(
                    &adj,
                    i,
                    i,
                    &mut visited,
                    &mut t,
                    &mut tin,
                    &mut low,
                    &mut res,
                );
            }
        }
        res
    }

    fn dfs(
        adj: &Vec<Vec<usize>>,
        u: usize,
        p: usize,
        visited: &mut Vec<bool>,
        t: &mut usize,
        tin: &mut Vec<usize>,
        low: &mut Vec<usize>,
        bridges: &mut Vec<Vec<i32>>,
    ) {
        tin[u] = *t;
        low[u] = *t;
        *t += 1;
        visited[u] = true;
        for &v in &adj[u] {
            if v == p {
                continue;
            }
            if visited[v] {
                low[u] = low[u].min(tin[v]);
            } else {
                Solution::dfs(adj, v, u, visited, t, tin, low, bridges);
                low[u] = low[u].min(low[v]);
                if low[v] > tin[u] {
                    bridges.push(vec![u as i32, v as i32]);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let n = 4;
        let connections = vec![vec![0, 1], vec![1, 2], vec![2, 0], vec![1, 3]];
        assert_eq!(
            Solution::critical_connections(n, connections),
            vec![vec![1, 3]]
        );

        let n = 2;
        let connections = vec![vec![0, 1]];
        assert_eq!(
            Solution::critical_connections(n, connections),
            vec![vec![0, 1]]
        );
    }
}
