struct Solution;

impl Solution {
    pub fn valid_path(n: i32, edges: Vec<Vec<i32>>, source: i32, destination: i32) -> bool {
        let mut adj: Vec<Vec<usize>> = vec![vec![]; n as usize];
        for e in edges.iter() {
            let (u, v) = (e[0] as usize, e[1] as usize);
            adj[u].push(v);
            adj[v].push(u);
        }
        Solution::dfs(
            &adj,
            &mut vec![false; n as usize],
            source as usize,
            destination as usize,
        )
    }

    fn dfs(adj: &Vec<Vec<usize>>, visited: &mut Vec<bool>, cur: usize, dest: usize) -> bool {
        if cur == dest {
            return true;
        }
        if visited[cur] {
            return false;
        }
        visited[cur] = true;
        for &next in &adj[cur] {
            if Solution::dfs(adj, visited, next, dest) {
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
        let n = 3;
        let edges = vec![vec![0, 1], vec![1, 2], vec![2, 0]];
        let source = 0;
        let destination = 2;
        assert!(Solution::valid_path(n, edges, source, destination));

        let n = 6;
        let edges = vec![vec![0, 1], vec![0, 2], vec![3, 5], vec![5, 4], vec![4, 3]];
        let source = 0;
        let destination = 5;
        assert!(!Solution::valid_path(n, edges, source, destination));
    }
}
