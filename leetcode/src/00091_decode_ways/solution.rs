struct Solution;

impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        let s = s.chars().collect::<Vec<_>>();
        let mut dp: Vec<Option<i32>> = vec![None; s.len() + 1];
        Solution::solve(&s, s.len(), &mut dp)
    }

    fn solve(s: &[char], n: usize, dp: &mut Vec<Option<i32>>) -> i32 {
        if n == 0 {
            return 1;
        }
        if let Some(val) = dp[n] {
            return val;
        }
        let mut res = 0;
        let last = s[n - 1] as u8 - b'0';
        if last != 0 {
            res += Solution::solve(s, n - 1, dp);
        }
        if n > 1 {
            let second_last = s[n - 2] as u8 - b'0';
            if second_last != 0 && second_last * 10 + last < 27 {
                res += Solution::solve(s, n - 2, dp);
            }
        }
        dp[n] = Some(res);
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let s = String::from("12");
        assert_eq!(Solution::num_decodings(s), 2);

        let s = String::from("226");
        assert_eq!(Solution::num_decodings(s), 3);

        let s = String::from("06");
        assert_eq!(Solution::num_decodings(s), 0);
    }
}
