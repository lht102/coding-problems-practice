struct Solution;

impl Solution {
    pub fn longest_subsequence(s: String, k: i32) -> i32 {
        let s = s
            .chars()
            .map(|ch| (ch as u8 - b'0') as i32)
            .rev()
            .collect::<Vec<_>>();
        let mut dp = vec![vec![-1; s.len() + 1]; s.len() + 1];
        Solution::solve(&s, k, 0, 0, &mut dp)
    }

    fn solve(s: &[i32], k: i32, cur_idx: usize, cur_len: usize, dp: &mut Vec<Vec<i32>>) -> i32 {
        if dp[cur_idx][cur_len] != -1 {
            return dp[cur_idx][cur_len];
        }
        if cur_idx == s.len() {
            return cur_len as i32;
        }
        let mut res = Solution::solve(s, k, cur_idx + 1, cur_len, dp);
        if let Some(val) = s[cur_idx].checked_shl(cur_len as u32) {
            if k >= val && val >= 0 {
                res = res.max(Solution::solve(s, k - val, cur_idx + 1, cur_len + 1, dp));
            }
        } else if s[cur_idx] == 0 {
            res = res.max(Solution::solve(s, k, cur_idx + 1, cur_len + 1, dp));
        }
        dp[cur_idx][cur_len] = res;
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let s = String::from("1001010");
        let k = 5;
        assert_eq!(Solution::longest_subsequence(s, k), 5);

        let s = String::from("00101001");
        let k = 1;
        assert_eq!(Solution::longest_subsequence(s, k), 6);

        let s = String::from(
            "111100010000011101001110001111000000001011101111111110111000011111011000010101110100110110001111001001011001010011010000011111101001101000000101101001110110000111101011000101",
        );
        let k = 11713332;
        assert_eq!(Solution::longest_subsequence(s, k), 96);
    }
}
