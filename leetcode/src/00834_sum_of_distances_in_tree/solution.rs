struct Solution;

impl Solution {
    pub fn sum_of_distances_in_tree(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        let n = n as usize;
        let mut adj = vec![vec![]; n];
        for edge in &edges {
            let u = edge[0] as usize;
            let v = edge[1] as usize;
            adj[u].push(v);
            adj[v].push(u);
        }
        let mut cnts = vec![0; n];
        let mut res = vec![0; n];
        Solution::dfs(&adj, 0, None, &mut cnts, &mut res);
        Solution::dfs2(&adj, 0, None, &mut cnts, &mut res);
        res
    }

    fn dfs(
        adj: &[Vec<usize>],
        source: usize,
        parent: Option<usize>,
        cnts: &mut Vec<i32>,
        res: &mut Vec<i32>,
    ) {
        for &destination in &adj[source] {
            if Some(destination) == parent {
                continue;
            }
            Solution::dfs(adj, destination, Some(source), cnts, res);
            cnts[source] += cnts[destination];
            res[source] += res[destination] + cnts[destination];
        }
        cnts[source] += 1;
    }

    fn dfs2(
        adj: &[Vec<usize>],
        source: usize,
        parent: Option<usize>,
        cnts: &[i32],
        res: &mut Vec<i32>,
    ) {
        for &destination in &adj[source] {
            if Some(destination) == parent {
                continue;
            }
            res[destination] =
                res[source] - cnts[destination] + adj.len() as i32 - cnts[destination];
            Solution::dfs2(adj, destination, Some(source), cnts, res);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let n = 6;
        let edges = vec![vec![0, 1], vec![0, 2], vec![2, 3], vec![2, 4], vec![2, 5]];
        assert_eq!(
            Solution::sum_of_distances_in_tree(n, edges),
            vec![8, 12, 6, 10, 10, 10]
        );

        let n = 1;
        let edges = vec![];
        assert_eq!(Solution::sum_of_distances_in_tree(n, edges), vec![0]);

        let n = 2;
        let edges = vec![vec![1, 0]];
        assert_eq!(Solution::sum_of_distances_in_tree(n, edges), vec![1, 1]);
    }
}
