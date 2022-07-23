use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn min_path_cost(grid: Vec<Vec<i32>>, move_cost: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let mut cell_to_row = HashMap::new();
        for (i, arr) in grid.iter().enumerate() {
            for &cell in arr {
                cell_to_row.insert(cell as usize, i);
            }
        }
        let start = m * n;
        let end = m * n + 1;
        let mut adj = vec![vec![]; m * n + 2];
        for (cell, mcs) in move_cost.iter().enumerate() {
            let row = *cell_to_row.get(&cell).unwrap();
            if row == 0 {
                adj[start].push((cell as i32, cell));
            } else if row == m - 1 {
                adj[cell].push((0, end));
                continue;
            }
            for (col, &mc) in mcs.iter().enumerate() {
                adj[cell].push((mc + grid[row + 1][col], grid[row + 1][col] as usize));
            }
        }
        let mut pq: BinaryHeap<(Reverse<i32>, usize)> = BinaryHeap::new();
        let mut dist = vec![1_000_000_000; m * n + 2];
        let mut vi = vec![false; m * n + 2];
        dist[start] = 0;
        pq.push((Reverse(dist[start]), start));
        while let Some((Reverse(d), u)) = pq.pop() {
            if u == end {
                return d;
            }
            if vi[u] {
                continue;
            }
            vi[u] = true;
            for &(cost, v) in &adj[u] {
                let new_cost = d + cost;
                if new_cost < dist[v] {
                    dist[v] = new_cost;
                    pq.push((Reverse(new_cost), v));
                }
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let grid = vec![vec![5, 3], vec![4, 0], vec![2, 1]];
        let move_cost = vec![
            vec![9, 8],
            vec![1, 5],
            vec![10, 12],
            vec![18, 6],
            vec![2, 4],
            vec![14, 3],
        ];
        assert_eq!(Solution::min_path_cost(grid, move_cost), 17);

        let grid = vec![vec![5, 1, 2], vec![4, 0, 3]];
        let move_cost = vec![
            vec![12, 10, 15],
            vec![20, 23, 8],
            vec![21, 7, 1],
            vec![8, 1, 13],
            vec![9, 10, 25],
            vec![5, 3, 2],
        ];
        assert_eq!(Solution::min_path_cost(grid, move_cost), 6);
    }
}
