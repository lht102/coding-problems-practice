struct Solution;

impl Solution {
    pub fn min_cost(n: i32, cuts: Vec<i32>) -> i32 {
        let mut new_cuts = cuts;
        new_cuts.push(0);
        new_cuts.push(n);
        new_cuts.sort_unstable();
        let mut dp = vec![vec![None; new_cuts.len()]; new_cuts.len()];
        Self::solve(&new_cuts, 0, new_cuts.len() - 1, &mut dp)
    }

    fn solve(new_cuts: &[i32], left: usize, right: usize, dp: &mut [Vec<Option<i32>>]) -> i32 {
        if right == 1 + left {
            return 0;
        }
        if let Some(val) = dp[left][right] {
            return val;
        }
        let mut res = i32::MAX;
        for mid in left + 1..right {
            res = res.min(
                Self::solve(new_cuts, left, mid, dp)
                    + Self::solve(new_cuts, mid, right, dp)
                    + new_cuts[right]
                    - new_cuts[left],
            );
        }
        dp[left][right] = Some(res);
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let n = 7;
        let cuts = vec![1, 3, 4, 5];
        assert_eq!(Solution::min_cost(n, cuts), 16);

        let n = 9;
        let cuts = vec![5, 6, 1, 4, 2];
        assert_eq!(Solution::min_cost(n, cuts), 22);
    }
}
