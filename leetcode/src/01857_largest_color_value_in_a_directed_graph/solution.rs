use std::collections::VecDeque;
use std::iter::FromIterator;

struct Solution;

impl Solution {
    pub fn largest_path_value(colors: String, edges: Vec<Vec<i32>>) -> i32 {
        let n = colors.len();
        let colors = colors.as_bytes();
        let mut adj = vec![vec![]; n];
        let mut indegrees = vec![0; n];
        for edge in &edges {
            let u = edge[0] as usize;
            let v = edge[1] as usize;
            adj[u].push(v);
            indegrees[v] += 1;
        }
        let mut q = VecDeque::from_iter(
            indegrees
                .iter()
                .enumerate()
                .filter_map(|(i, &indegree)| (indegree == 0).then_some(i)),
        );
        let mut freq = vec![vec![0; 26]; n];
        let mut res = 0;
        let mut seen = 0;
        while let Some(cur_node) = q.pop_front() {
            let idx = (colors[cur_node] - b'a') as usize;
            freq[cur_node][idx] += 1;
            res = res.max(freq[cur_node][idx]);
            seen += 1;
            for &next_node in &adj[cur_node] {
                for i in 0..26 {
                    freq[next_node][i] = freq[next_node][i].max(freq[cur_node][i]);
                }
                indegrees[next_node] -= 1;
                if indegrees[next_node] == 0 {
                    q.push_back(next_node);
                }
            }
        }
        if seen < n { -1 } else { res }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let colors = String::from("abaca");
        let edges = vec![vec![0, 1], vec![0, 2], vec![2, 3], vec![3, 4]];
        assert_eq!(Solution::largest_path_value(colors, edges), 3);

        let colors = String::from("a");
        let edges = vec![vec![0, 0]];
        assert_eq!(Solution::largest_path_value(colors, edges), -1);
    }
}
