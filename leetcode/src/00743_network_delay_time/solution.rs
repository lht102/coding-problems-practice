use std::cmp::Reverse;
use std::collections::BinaryHeap;

struct Solution;

impl Solution {
    pub fn network_delay_time(times: Vec<Vec<i32>>, n: i32, k: i32) -> i32 {
        let n = n as usize;
        let k = k as usize;

        let mut adj: Vec<Vec<(i32, usize)>> = vec![vec![]; n + 1];
        for t in &times {
            adj[t[0] as usize].push((t[2], t[1] as usize));
        }

        let mut dist = vec![1_000_000_000; n + 1];
        let mut vi = vec![false; n + 1];
        let mut pq: BinaryHeap<Reverse<(i32, usize)>> = BinaryHeap::new();
        dist[k] = 0;
        pq.push(Reverse((dist[k], k)));
        while let Some(Reverse((d, u))) = pq.pop() {
            if vi[u] {
                continue;
            }
            vi[u] = true;
            for &(cost, v) in &adj[u] {
                let new_cost = d + cost;
                if new_cost < dist[v] {
                    dist[v] = new_cost;
                    pq.push(Reverse((new_cost, v)));
                }
            }
        }

        if vi.iter().skip(1).any(|&visited| !visited) {
            -1
        } else {
            *dist.iter().skip(1).max().unwrap()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let times = vec![vec![2, 1, 1], vec![2, 3, 1], vec![3, 4, 1]];
        let n = 4;
        let k = 2;
        assert_eq!(Solution::network_delay_time(times, n, k), 2);

        let times = vec![vec![1, 2, 1]];
        let n = 2;
        let k = 1;
        assert_eq!(Solution::network_delay_time(times, n, k), 1);

        let times = vec![vec![1, 2, 1]];
        let n = 2;
        let k = 2;
        assert_eq!(Solution::network_delay_time(times, n, k), -1);
    }
}
