use std::cmp::Ordering;

struct Solution;

impl Solution {
    pub fn stone_game_iii(stone_value: Vec<i32>) -> String {
        let suf = {
            let mut v = stone_value
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
        let mut dp = vec![None; stone_value.len()];
        let val = Self::solve(&stone_value, 0, &suf, &mut dp) * 2 - suf[0];
        match val.cmp(&0) {
            Ordering::Less => String::from("Bob"),
            Ordering::Equal => String::from("Tie"),
            Ordering::Greater => String::from("Alice"),
        }
    }

    fn solve(piles: &[i32], i: usize, suf: &[i32], dp: &mut [Option<i32>]) -> i32 {
        if i >= piles.len() {
            return 0;
        }
        if let Some(val) = dp[i] {
            return val;
        }
        let mut min = i32::MAX;
        for j in 1..=3 {
            min = min.min(Self::solve(piles, i + j, suf, dp));
        }
        let res = suf[i] - min;
        dp[i] = Some(res);
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let stone_value = vec![1, 2, 3, 7];
        assert_eq!(Solution::stone_game_iii(stone_value), String::from("Bob"));

        let stone_value = vec![1, 2, 3, -9];
        assert_eq!(Solution::stone_game_iii(stone_value), String::from("Alice"));

        let stone_value = vec![1, 2, 3, 6];
        assert_eq!(Solution::stone_game_iii(stone_value), String::from("Tie"));

        let stone_value = vec![-1, -2, -3];
        assert_eq!(Solution::stone_game_iii(stone_value), String::from("Tie"));

        let stone_value = vec![
            9, -4, 0, 12, -5, -13, 15, 6, -16, 8, 2, 16, 12, -6, 13, 0, -16, -11, 9, -14, 7, -1, 14,
        ];
        assert_eq!(Solution::stone_game_iii(stone_value), String::from("Bob"));
    }
}
