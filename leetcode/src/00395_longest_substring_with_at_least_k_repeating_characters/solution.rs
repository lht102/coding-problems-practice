struct Solution;

impl Solution {
    pub fn longest_substring(s: String, k: i32) -> i32 {
        let s = s.as_bytes();
        let k = k as usize;
        let n = s.len();
        let mut res = 0;
        let max_unique_cnt = Self::get_max_unique_count(s);
        for cur_unique_cnt in 1..=max_unique_cnt {
            let mut freq = vec![0; 26];
            let (mut start, mut end) = (0, 0);
            let (mut unique_cnt, mut at_least_k_cnt) = (0, 0);
            while end < n {
                if unique_cnt <= cur_unique_cnt {
                    let idx = Self::get_index(s[end]);
                    if freq[idx] == 0 {
                        unique_cnt += 1;
                    }
                    freq[idx] += 1;
                    if freq[idx] == k {
                        at_least_k_cnt += 1;
                    }
                    end += 1;
                } else {
                    let idx = Self::get_index(s[start]);
                    if freq[idx] == k {
                        at_least_k_cnt -= 1;
                    }
                    freq[idx] -= 1;
                    if freq[idx] == 0 {
                        unique_cnt -= 1;
                    }
                    start += 1;
                }
                if unique_cnt == cur_unique_cnt && unique_cnt == at_least_k_cnt {
                    res = res.max(end - start);
                }
            }
        }
        res as _
    }

    fn get_max_unique_count(s: &[u8]) -> usize {
        let mut res = 0;
        let mut seen = vec![false; 26];
        for i in s.iter().copied().map(Self::get_index) {
            if !seen[i] {
                res += 1;
                seen[i] = true;
            }
        }
        res
    }

    fn get_index(ch: u8) -> usize {
        (ch - b'a') as _
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let s = String::from("aaabb");
        let k = 3;
        assert_eq!(Solution::longest_substring(s, k), 3);

        let s = String::from("ababbc");
        let k = 2;
        assert_eq!(Solution::longest_substring(s, k), 5);
    }
}
