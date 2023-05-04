use std::cmp::Reverse;
use std::collections::BinaryHeap;

struct Solution;

impl Solution {
    pub fn minimum_cost(start: Vec<i32>, target: Vec<i32>, special_roads: Vec<Vec<i32>>) -> i32 {
        let n = special_roads.len();
        let mut dist = vec![1_000_000_000; n];
        let mut pq = BinaryHeap::new();
        for (i, road) in special_roads.iter().enumerate() {
            dist[i] = Self::calculate_cost(start[0], start[1], road[0], road[1]) + road[4];
            pq.push(Reverse((dist[i], i)));
        }
        let mut res = Self::calculate_cost(start[0], start[1], target[0], target[1]);
        while let Some(Reverse((total_cost, i))) = pq.pop() {
            res = res.min(
                dist[i]
                    + Self::calculate_cost(
                        special_roads[i][2],
                        special_roads[i][3],
                        target[0],
                        target[1],
                    ),
            );
            for (j, road) in special_roads.iter().enumerate() {
                if i == j {
                    continue;
                }
                let cost = Self::calculate_cost(
                    special_roads[i][2],
                    special_roads[i][3],
                    road[0],
                    road[1],
                ) + road[4];
                let new_total_cost = total_cost + cost;
                if new_total_cost < dist[j] {
                    dist[j] = new_total_cost;
                    pq.push(Reverse((new_total_cost, j)));
                }
            }
        }
        res
    }

    fn calculate_cost(x1: i32, y1: i32, x2: i32, y2: i32) -> i32 {
        (x2 - x1).abs() + (y2 - y1).abs()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let start = vec![1, 1];
        let target = vec![4, 5];
        let special_roads = vec![vec![1, 2, 3, 3, 2], vec![3, 4, 4, 5, 1]];
        assert_eq!(Solution::minimum_cost(start, target, special_roads), 5);

        let start = vec![3, 2];
        let target = vec![5, 7];
        let special_roads = vec![
            vec![3, 2, 3, 4, 4],
            vec![3, 3, 5, 5, 5],
            vec![3, 4, 5, 6, 6],
        ];
        assert_eq!(Solution::minimum_cost(start, target, special_roads), 7);
    }
}
