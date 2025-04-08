use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn get_length_of_optimal_compression(s: String, k: i32) -> i32 {
        Solution::solve(s.as_bytes(), k as usize, 0, 0, 0, &mut HashMap::new())
    }

    fn solve(
        s: &[u8],
        k: usize,
        i: usize,
        prev_ch: u8,
        prev_cnt: usize,
        dp: &mut HashMap<(usize, usize, u8, usize), i32>,
    ) -> i32 {
        if i == s.len() {
            return Solution::calculate_len(prev_cnt);
        }
        if let Some(val) = dp.get(&(i, k, prev_ch, prev_cnt)) {
            return *val;
        }
        let delete = if k > 0 {
            Solution::solve(s, k - 1, i + 1, prev_ch, prev_cnt, dp)
        } else {
            i32::MAX
        };
        let keep = if s[i] == prev_ch {
            Solution::solve(s, k, i + 1, prev_ch, prev_cnt + 1, dp)
        } else {
            Solution::solve(s, k, i + 1, s[i], 1, dp) + Solution::calculate_len(prev_cnt)
        };
        let res = delete.min(keep);
        dp.insert((i, k, prev_ch, prev_cnt), res);
        res
    }

    fn calculate_len(cnt: usize) -> i32 {
        if cnt == 0 {
            0
        } else if cnt == 1 {
            1
        } else if cnt < 10 {
            2
        } else if cnt < 100 {
            3
        } else {
            4
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let s = String::from("aaabcccd");
        let k = 2;
        assert_eq!(Solution::get_length_of_optimal_compression(s, k), 4);

        let s = String::from("aabbaa");
        let k = 2;
        assert_eq!(Solution::get_length_of_optimal_compression(s, k), 2);

        let s = String::from("aaaaaaaaaaa");
        let k = 0;
        assert_eq!(Solution::get_length_of_optimal_compression(s, k), 3);

        let s = String::from(
            "bccomlcafhldoebfbgmiiicfhdfgfmggmlfjakbhfmolkajncdofofokngifmedleoelbhhmgiaibkgccifbmkolkjhifidobido",
        );
        let k = 47;
        assert_eq!(Solution::get_length_of_optimal_compression(s, k), 41);
    }
}
