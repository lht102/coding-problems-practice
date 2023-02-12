struct Solution;

impl Solution {
    pub fn minimum_fuel_cost(roads: Vec<Vec<i32>>, seats: i32) -> i64 {
        let n = roads.len() + 1;
        let mut adj = vec![vec![]; n];
        for road in &roads {
            let (u, v) = (road[0] as usize, road[1] as usize);
            adj[u].push(v);
            adj[v].push(u);
        }
        let mut res = 0;
        Self::dfs(&adj, 0, None, seats as _, &mut res);
        res
    }

    fn dfs(
        adj: &[Vec<usize>],
        cur: usize,
        parent: Option<usize>,
        seats: usize,
        fuels: &mut i64,
    ) -> usize {
        let mut representatives = 1;
        for &next in &adj[cur] {
            if parent == Some(next) {
                continue;
            }
            representatives += Self::dfs(adj, next, Some(cur), seats, fuels);
        }
        if cur != 0 {
            *fuels += ((representatives as f64) / (seats as f64)).ceil() as i64;
        }
        representatives
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let roads = vec![vec![0, 1], vec![0, 2], vec![0, 3]];
        let seats = 5;
        assert_eq!(Solution::minimum_fuel_cost(roads, seats), 3);

        let roads = vec![
            vec![3, 1],
            vec![3, 2],
            vec![1, 0],
            vec![0, 4],
            vec![0, 5],
            vec![4, 6],
        ];
        let seats = 2;
        assert_eq!(Solution::minimum_fuel_cost(roads, seats), 7);

        let roads = vec![];
        let seats = 1;
        assert_eq!(Solution::minimum_fuel_cost(roads, seats), 0);
    }
}
