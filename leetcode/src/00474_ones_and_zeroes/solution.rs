struct Solution;

impl Solution {
    pub fn find_max_form(strs: Vec<String>, m: i32, n: i32) -> i32 {
        let zeros = strs
            .iter()
            .map(|s| s.chars().filter(|&ch| ch == '0').count() as usize)
            .collect::<Vec<_>>();
        let mut dp = vec![vec![vec![-1; n as usize + 1]; m as usize + 1]; strs.len()];
        Solution::solve(&strs, &zeros, strs.len() - 1, m as i32, n as i32, &mut dp)
    }

    fn solve(
        strs: &Vec<String>,
        zeros: &[usize],
        k: usize,
        m: i32,
        n: i32,
        dp: &mut Vec<Vec<Vec<i32>>>,
    ) -> i32 {
        if k == 0 {
            if m - zeros[0] as i32 >= 0 && n - (strs[0].len() - zeros[0]) as i32 >= 0 {
                return 1;
            } else {
                return 0;
            }
        }
        if dp[k][m as usize][n as usize] != -1 {
            return dp[k][m as usize][n as usize];
        }

        dp[k][m as usize][n as usize] =
            if m - zeros[k] as i32 >= 0 && n - (strs[k].len() - zeros[k]) as i32 >= 0 {
                Solution::solve(strs, zeros, k - 1, m, n, dp).max(
                    Solution::solve(
                        strs,
                        zeros,
                        k - 1,
                        m - zeros[k] as i32,
                        n - (strs[k].len() - zeros[k]) as i32,
                        dp,
                    ) + 1,
                )
            } else {
                Solution::solve(strs, zeros, k - 1, m, n, dp)
            };
        dp[k][m as usize][n as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let strs = vec![
            String::from("10"),
            String::from("0001"),
            String::from("111001"),
            String::from("1"),
            String::from("0"),
        ];
        let m = 5;
        let n = 3;
        assert_eq!(Solution::find_max_form(strs, m, n), 4);

        let strs = vec![String::from("10"), String::from("0"), String::from("1")];
        let m = 1;
        let n = 1;
        assert_eq!(Solution::find_max_form(strs, m, n), 2);

        let strs = vec![
            String::from("0"),
            String::from("0"),
            String::from("1"),
            String::from("1"),
        ];
        let m = 2;
        let n = 2;
        assert_eq!(Solution::find_max_form(strs, m, n), 4);
    }
}
