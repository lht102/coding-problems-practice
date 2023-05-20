use std::collections::VecDeque;

struct Solution;

impl Solution {
    pub fn is_bipartite(graph: Vec<Vec<i32>>) -> bool {
        let graph = graph
            .into_iter()
            .map(|x| x.into_iter().map(|y| y as usize).collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let mut colors = vec![None; graph.len()];
        for i in 0..colors.len() {
            if colors[i].is_some() {
                continue;
            }
            colors[i] = Some(true);
            let mut q = VecDeque::from([i]);
            while let Some(u) = q.pop_front() {
                for &v in &graph[u] {
                    if colors[u] == colors[v] {
                        return false;
                    }
                    if colors[v].is_none() {
                        colors[v] = colors[u].map(|x| !x);
                        q.push_back(v);
                    }
                }
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let graph = vec![vec![1, 2, 3], vec![0, 2], vec![0, 1, 3], vec![0, 2]];
        assert!(!Solution::is_bipartite(graph));

        let graph = vec![vec![1, 3], vec![0, 2], vec![1, 3], vec![0, 2]];
        assert!(Solution::is_bipartite(graph));
    }
}
