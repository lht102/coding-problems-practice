use std::collections::VecDeque;

struct Solution;

impl Solution {
    pub fn possible_bipartition(n: i32, dislikes: Vec<Vec<i32>>) -> bool {
        let mut adj = vec![vec![]; n as usize];
        for dislike in &dislikes {
            let p1 = (dislike[0] - 1) as usize;
            let p2 = (dislike[1] - 1) as usize;
            adj[p1].push(p2);
            adj[p2].push(p1);
        }
        Solution::bfs(&adj)
    }

    fn bfs(adj: &Vec<Vec<usize>>) -> bool {
        let mut q = VecDeque::new();
        let mut colors = vec![-1; adj.len()];
        for i in 0..adj.len() {
            if colors[i] != -1 {
                continue;
            }
            colors[i] = 0;
            q.push_back(i);
            while let Some(cur) = q.pop_front() {
                for &next in &adj[cur] {
                    if colors[cur] == colors[next] {
                        return false;
                    }
                    if colors[next] != -1 {
                        continue;
                    }
                    colors[next] = colors[cur] ^ 1;
                    q.push_back(next);
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
        let n = 4;
        let dislikes = vec![vec![1, 2], vec![1, 3], vec![2, 4]];
        assert!(Solution::possible_bipartition(n, dislikes));

        // let n = 3;
        // let dislikes = vec![vec![1, 2], vec![1, 3], vec![2, 3]];
        // assert!(!Solution::possible_bipartition(n, dislikes));
        //
        // let n = 5;
        // let dislikes = vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5], vec![1, 5]];
        // assert!(!Solution::possible_bipartition(n, dislikes));
    }
}
