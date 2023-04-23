struct Solution;

impl Solution {
    pub fn number_of_arrays(s: String, k: i32) -> i32 {
        let mut dp = vec![None; s.len()];
        Self::solve(s.as_bytes(), k as i64, 0, &mut dp) as _
    }

    fn solve(s: &[u8], k: i64, i: usize, dp: &mut [Option<i64>]) -> i64 {
        if i == s.len() {
            return 1;
        }
        if s[i] == b'0' {
            return 0;
        }
        if let Some(val) = dp[i] {
            return val;
        }
        const M: i64 = 1_000_000_007;
        let mut res = 0;
        let mut num = 0;
        for (j, ch) in s.iter().enumerate().skip(i) {
            num = num * 10 + (ch - b'0') as i64;
            if num > k {
                break;
            }
            res = (res + Self::solve(s, k, j + 1, dp)) % M;
        }
        dp[i] = Some(res);
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let s = String::from("1000");
        let k = 10000;
        assert_eq!(Solution::number_of_arrays(s, k), 1);

        let s = String::from("1000");
        let k = 10;
        assert_eq!(Solution::number_of_arrays(s, k), 0);

        let s = String::from("1317");
        let k = 2000;
        assert_eq!(Solution::number_of_arrays(s, k), 8);
    }
}
