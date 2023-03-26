use std::collections::VecDeque;

struct Solution;

impl Solution {
    pub fn longest_cycle(edges: Vec<i32>) -> i32 {
        let n = edges.len();
        let edges = edges
            .into_iter()
            .map(|edge| (edge != -1).then_some(edge as usize))
            .collect::<Vec<_>>();
        let mut indegrees = vec![0; n];
        let mut adj = vec![vec![]; n];
        for (u, &v) in edges.iter().enumerate() {
            if let Some(v) = v {
                adj[u].push(v);
                indegrees[v] += 1;
            }
        }

        let mut q = VecDeque::new();
        let mut visited = vec![false; n];
        for (i, &indegree) in indegrees.iter().enumerate() {
            if indegree == 0 {
                q.push_back(i);
                visited[i] = true;
            }
        }
        while let Some(cur) = q.pop_front() {
            for &next in &adj[cur] {
                indegrees[next] -= 1;
                if indegrees[next] != 0 {
                    continue;
                }
                visited[next] = true;
                q.push_back(next);
            }
        }

        let mut res = -1;
        for i in 0..n {
            if visited[i] {
                continue;
            }
            res = res.max(Self::get_cycle_len(&edges, &mut visited, i));
        }
        res
    }

    fn get_cycle_len(edges: &[Option<usize>], visited: &mut [bool], source: usize) -> i32 {
        let mut res = 0;
        let mut idx = source;
        visited[idx] = true;
        loop {
            idx = edges[idx].unwrap();
            visited[idx] = true;
            res += 1;
            if idx == source {
                break;
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let edges = vec![3, 3, 4, 2, 3];
        assert_eq!(Solution::longest_cycle(edges), 3);

        let edges = vec![2, -1, 3, 1];
        assert_eq!(Solution::longest_cycle(edges), -1);
    }
}
