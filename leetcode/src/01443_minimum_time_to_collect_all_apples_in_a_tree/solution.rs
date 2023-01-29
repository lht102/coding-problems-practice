struct Solution;

impl Solution {
    pub fn min_time(n: i32, edges: Vec<Vec<i32>>, has_apple: Vec<bool>) -> i32 {
        let n = n as usize;
        let mut adj = vec![vec![]; n];
        for edge in &edges {
            let u = edge[0] as usize;
            let v = edge[1] as usize;
            adj[u].push(v);
            adj[v].push(u);
        }
        Solution::dfs(0, None, &adj, &has_apple)
    }

    fn dfs(source: usize, parent: Option<usize>, adj: &[Vec<usize>], has_apple: &[bool]) -> i32 {
        let mut total_time = 0;
        for &destination in &adj[source] {
            if Some(destination) == parent {
                continue;
            }
            let t = Solution::dfs(destination, Some(source), adj, has_apple);
            if t > 0 || has_apple[destination] {
                total_time += t + 2;
            }
        }
        total_time
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let n = 7;
        let edges = vec![
            vec![0, 1],
            vec![0, 2],
            vec![1, 4],
            vec![1, 5],
            vec![2, 3],
            vec![2, 6],
        ];
        let has_apple = vec![false, false, true, false, true, true, false];
        assert_eq!(Solution::min_time(n, edges, has_apple), 8);

        let n = 7;
        let edges = vec![
            vec![0, 1],
            vec![0, 2],
            vec![1, 4],
            vec![1, 5],
            vec![2, 3],
            vec![2, 6],
        ];
        let has_apple = vec![false, false, true, false, false, true, false];
        assert_eq!(Solution::min_time(n, edges, has_apple), 6);

        let n = 7;
        let edges = vec![
            vec![0, 1],
            vec![0, 2],
            vec![1, 4],
            vec![1, 5],
            vec![2, 3],
            vec![2, 6],
        ];
        let has_apple = vec![false, false, false, false, false, false, false];
        assert_eq!(Solution::min_time(n, edges, has_apple), 0);
    }
}
