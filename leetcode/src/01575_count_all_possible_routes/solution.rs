struct Solution;

impl Solution {
    pub fn count_routes(locations: Vec<i32>, start: i32, finish: i32, fuel: i32) -> i32 {
        let mut dp = vec![vec![None; fuel as usize + 1]; locations.len()];
        Self::solve(&locations, start as _, finish as _, fuel, &mut dp)
    }

    fn solve(
        locations: &[i32],
        i: usize,
        finish: usize,
        fuel: i32,
        dp: &mut [Vec<Option<i32>>],
    ) -> i32 {
        if fuel < 0 {
            return 0;
        }
        let f = fuel as usize;
        if let Some(val) = dp[i][f] {
            return val;
        }
        let mut res = if i == finish { 1 } else { 0 };
        for (j, &loc) in locations.iter().enumerate().filter(|&(j, _)| i != j) {
            res = (res + Self::solve(locations, j, finish, fuel - (locations[i] - loc).abs(), dp))
                % 1_000_000_007;
        }
        dp[i][f] = Some(res);
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let locations = vec![2, 3, 6, 8, 4];
        let start = 1;
        let finish = 3;
        let fuel = 5;
        assert_eq!(Solution::count_routes(locations, start, finish, fuel), 4);

        let locations = vec![4, 3, 1];
        let start = 1;
        let finish = 0;
        let fuel = 6;
        assert_eq!(Solution::count_routes(locations, start, finish, fuel), 5);

        let locations = vec![5, 2, 1];
        let start = 0;
        let finish = 2;
        let fuel = 3;
        assert_eq!(Solution::count_routes(locations, start, finish, fuel), 0);

        let locations = vec![5, 2, 1];
        let start = 0;
        let finish = 0;
        let fuel = 20;
        assert_eq!(Solution::count_routes(locations, start, finish, fuel), 172);
    }
}
