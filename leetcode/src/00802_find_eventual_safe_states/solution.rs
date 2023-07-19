use std::collections::VecDeque;
use std::iter::FromIterator;

struct Solution;

impl Solution {
    pub fn eventual_safe_nodes(graph: Vec<Vec<i32>>) -> Vec<i32> {
        let n = graph.len();
        let mut g = vec![vec![]; n];
        let mut indegrees = vec![0; n];
        for (i, edges) in graph.iter().enumerate() {
            for e in edges.iter().map(|&x| x as usize) {
                g[e].push(i);
                indegrees[i] += 1;
            }
        }
        let mut q = VecDeque::from_iter(
            indegrees
                .iter()
                .enumerate()
                .filter_map(|(i, &indegree)| (indegree == 0).then_some(i)),
        );
        let mut visited = vec![false; n];
        while let Some(u) = q.pop_front() {
            visited[u] = true;
            for &v in &g[u] {
                indegrees[v] -= 1;
                if indegrees[v] == 0 {
                    q.push_back(v);
                }
            }
        }
        visited
            .into_iter()
            .enumerate()
            .filter_map(|(i, x)| x.then_some(i as i32))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let graph = vec![
            vec![1, 2],
            vec![2, 3],
            vec![5],
            vec![0],
            vec![5],
            vec![],
            vec![],
        ];
        assert_eq!(Solution::eventual_safe_nodes(graph), vec![2, 4, 5, 6]);

        let graph = vec![vec![1, 2, 3, 4], vec![1, 2], vec![3, 4], vec![0, 4], vec![]];
        assert_eq!(Solution::eventual_safe_nodes(graph), vec![4]);
    }
}
