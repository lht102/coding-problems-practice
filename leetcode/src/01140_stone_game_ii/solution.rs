struct Solution;

impl Solution {
    pub fn stone_game_ii(piles: Vec<i32>) -> i32 {
        let suf = {
            let mut v = piles
                .iter()
                .rev()
                .scan(0, |acc, x| {
                    *acc += x;
                    Some(*acc)
                })
                .collect::<Vec<_>>();
            v.reverse();
            v
        };
        let mut dp = vec![vec![None; piles.len()]; piles.len()];
        Self::solve(&piles, 0, 1, &suf, &mut dp)
    }

    fn solve(piles: &[i32], i: usize, m: usize, suf: &[i32], dp: &mut [Vec<Option<i32>>]) -> i32 {
        if i == piles.len() {
            return 0;
        }
        if i + 2 * m >= piles.len() {
            return suf[i];
        }
        if let Some(val) = dp[i][m] {
            return val;
        }
        let mut min = i32::MAX;
        for x in 1..=2 * m {
            min = min.min(Self::solve(piles, i + x, m.max(x), suf, dp));
        }
        let res = suf[i] - min;
        dp[i][m] = Some(res);
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let piles = vec![2, 7, 9, 4, 4];
        assert_eq!(Solution::stone_game_ii(piles), 10);

        let piles = vec![1, 2, 3, 4, 5, 100];
        assert_eq!(Solution::stone_game_ii(piles), 104);
    }
}
