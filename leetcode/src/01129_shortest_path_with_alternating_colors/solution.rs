use std::collections::VecDeque;

struct Solution;

#[derive(Copy, Clone, PartialEq)]
enum Color {
    Red,
    Blue,
}

impl Solution {
    pub fn shortest_alternating_paths(
        n: i32,
        red_edges: Vec<Vec<i32>>,
        blue_edges: Vec<Vec<i32>>,
    ) -> Vec<i32> {
        let n = n as usize;
        let mut adj = vec![vec![]; n];
        for edge in &red_edges {
            adj[edge[0] as usize].push((edge[1] as usize, Color::Red));
        }
        for edge in &blue_edges {
            adj[edge[0] as usize].push((edge[1] as usize, Color::Blue));
        }
        let mut res: Vec<Option<usize>> = vec![None; n];
        res[0] = Some(0);
        let mut visited: Vec<Vec<bool>> = vec![vec![false; 2]; n];
        visited[0][Color::Red as usize] = true;
        visited[0][Color::Blue as usize] = true;
        let mut q: VecDeque<(usize, usize, Option<Color>)> = VecDeque::from([(0, 0, None)]);
        while let Some((cur_node, cur_length, cur_color)) = q.pop_front() {
            for &(next_node, next_color) in &adj[cur_node] {
                if visited[next_node][next_color as usize] || cur_color == Some(next_color) {
                    continue;
                }
                visited[next_node][next_color as usize] = true;
                let next_length = cur_length + 1;
                q.push_back((next_node, next_length, Some(next_color)));
                if res[next_node].is_none() {
                    res[next_node] = Some(next_length);
                }
            }
        }
        res.into_iter()
            .map(|length| length.map_or(-1, |length| length as _))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let n = 3;
        let red_edges = vec![vec![0, 1], vec![1, 2]];
        let blue_edges = vec![];
        assert_eq!(
            Solution::shortest_alternating_paths(n, red_edges, blue_edges),
            vec![0, 1, -1]
        );

        let n = 3;
        let red_edges = vec![vec![0, 1]];
        let blue_edges = vec![vec![2, 1]];
        assert_eq!(
            Solution::shortest_alternating_paths(n, red_edges, blue_edges),
            vec![0, 1, -1]
        );

        let n = 5;
        let red_edges = vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![3, 4]];
        let blue_edges = vec![vec![1, 2], vec![2, 3], vec![3, 1]];
        assert_eq!(
            Solution::shortest_alternating_paths(n, red_edges, blue_edges),
            vec![0, 1, 2, 3, 7]
        );

        let n = 3;
        let red_edges = vec![vec![0, 1], vec![0, 2]];
        let blue_edges = vec![vec![1, 0]];
        assert_eq!(
            Solution::shortest_alternating_paths(n, red_edges, blue_edges),
            vec![0, 1, 1]
        );

        let n = 7;
        let red_edges = vec![
            vec![5, 5],
            vec![4, 6],
            vec![6, 4],
            vec![4, 1],
            vec![4, 3],
            vec![2, 3],
            vec![1, 1],
            vec![1, 6],
            vec![0, 6],
            vec![0, 2],
            vec![1, 5],
            vec![6, 0],
        ];
        let blue_edges = vec![
            vec![1, 1],
            vec![5, 2],
            vec![4, 0],
            vec![5, 5],
            vec![2, 4],
            vec![4, 2],
            vec![0, 4],
            vec![0, 5],
            vec![3, 5],
            vec![1, 2],
            vec![6, 2],
            vec![0, 6],
            vec![3, 1],
            vec![5, 0],
        ];
        assert_eq!(
            Solution::shortest_alternating_paths(n, red_edges, blue_edges),
            vec![0, 2, 1, 2, 1, 1, 1]
        );
    }
}
