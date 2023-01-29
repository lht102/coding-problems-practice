use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn count_sub_trees(n: i32, edges: Vec<Vec<i32>>, labels: String) -> Vec<i32> {
        let n = n as usize;
        let mut adj = vec![vec![]; n];
        for edge in &edges {
            let u = edge[0] as usize;
            let v = edge[1] as usize;
            adj[u].push(v);
            adj[v].push(u);
        }
        let mut res = vec![0; n];
        Solution::dfs(&adj, 0, None, &labels.chars().collect::<Vec<_>>(), &mut res);
        res
    }

    fn dfs(
        adj: &[Vec<usize>],
        cur: usize,
        parent: Option<usize>,
        labels: &[char],
        res: &mut Vec<i32>,
    ) -> HashMap<char, usize> {
        let mut cnts = HashMap::<char, usize>::with_capacity(26);
        cnts.insert(labels[cur], 1);
        for &next in &adj[cur] {
            if Some(next) == parent {
                continue;
            }
            let child_cnts = Solution::dfs(adj, next, Some(cur), labels, res);
            for (ch, cnt) in child_cnts {
                *cnts.entry(ch).or_default() += cnt;
            }
        }
        res[cur] = *cnts.get(&labels[cur]).unwrap() as i32;
        cnts
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
        let labels = String::from("abaedcd");
        assert_eq!(
            Solution::count_sub_trees(n, edges, labels),
            vec![2, 1, 1, 1, 1, 1, 1]
        );

        let n = 4;
        let edges = vec![vec![0, 1], vec![1, 2], vec![0, 3]];
        let labels = String::from("bbbb");
        assert_eq!(
            Solution::count_sub_trees(n, edges, labels),
            vec![4, 2, 1, 1]
        );
    }
}
