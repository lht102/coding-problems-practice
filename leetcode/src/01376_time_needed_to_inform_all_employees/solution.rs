use std::collections::VecDeque;

struct Solution;

impl Solution {
    pub fn num_of_minutes(n: i32, head_id: i32, manager: Vec<i32>, inform_time: Vec<i32>) -> i32 {
        let n = n as usize;
        let adj = manager.iter().enumerate().filter(|&(_, &m)| m != -1).fold(
            vec![vec![]; n],
            |mut g, (i, &m)| {
                g[m as usize].push(i);
                g
            },
        );
        let mut q = VecDeque::from([(head_id as usize, 0)]);
        let mut res = 0;
        while let Some((u, t)) = q.pop_front() {
            res = res.max(t);
            for &v in &adj[u] {
                q.push_back((v, t + inform_time[u]));
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
        let n = 1;
        let head_id = 0;
        let manager = vec![-1];
        let inform_time = vec![0];
        assert_eq!(
            Solution::num_of_minutes(n, head_id, manager, inform_time),
            0,
        );

        let n = 6;
        let head_id = 2;
        let manager = vec![2, 2, -1, 2, 2, 2];
        let inform_time = vec![0, 0, 1, 0, 0, 0];
        assert_eq!(
            Solution::num_of_minutes(n, head_id, manager, inform_time),
            1,
        );

        let n = 15;
        let head_id = 0;
        let manager = vec![-1, 0, 0, 1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6];
        let inform_time = vec![1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0];
        assert_eq!(
            Solution::num_of_minutes(n, head_id, manager, inform_time),
            3,
        );
    }
}
