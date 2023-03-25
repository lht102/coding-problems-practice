use std::collections::VecDeque;

struct Solution;

impl Solution {
    pub fn min_score(n: i32, roads: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut adj = vec![vec![]; n + 1];
        for road in roads {
            let u = road[0] as usize;
            let v = road[1] as usize;
            let distance = road[2];
            adj[u].push((distance, v));
            adj[v].push((distance, u));
        }

        let mut res = i32::MAX;
        let mut visited = vec![false; n + 1];
        visited[1] = true;
        let mut q = VecDeque::from([1]);
        while let Some(cur_node) = q.pop_front() {
            for &(distance, next_node) in &adj[cur_node] {
                res = res.min(distance);
                if visited[next_node] {
                    continue;
                }
                q.push_back(next_node);
                visited[next_node] = true;
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
        let n = 4;
        let roads = vec![vec![1, 2, 9], vec![2, 3, 6], vec![2, 4, 5], vec![1, 4, 7]];
        assert_eq!(Solution::min_score(n, roads), 5);

        let n = 4;
        let roads = vec![vec![1, 2, 2], vec![1, 3, 4], vec![3, 4, 7]];
        assert_eq!(Solution::min_score(n, roads), 2);
    }
}
