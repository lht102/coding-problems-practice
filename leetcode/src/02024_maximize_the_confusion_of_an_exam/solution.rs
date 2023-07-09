struct Solution;

impl Solution {
    pub fn max_consecutive_answers(answer_key: String, k: i32) -> i32 {
        let answer_key = answer_key.chars().collect::<Vec<_>>();
        let k = k as usize;
        Self::max_consecutive(&answer_key, k, 'T').max(Self::max_consecutive(&answer_key, k, 'F'))
            as _
    }

    fn max_consecutive(answer_key: &[char], k: usize, target: char) -> usize {
        let mut res = 0;
        let mut target_cnt = 0;
        let mut left = 0;
        for right in 0..answer_key.len() {
            if answer_key[right] == target {
                target_cnt += 1;
            }
            while target_cnt > k {
                if answer_key[left] == target {
                    target_cnt -= 1;
                }
                left += 1;
            }
            res = res.max(right - left + 1);
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let answer_key = String::from("TTFF");
        let k = 2;
        assert_eq!(Solution::max_consecutive_answers(answer_key, k), 4);

        let answer_key = String::from("TFFT");
        let k = 1;
        assert_eq!(Solution::max_consecutive_answers(answer_key, k), 3);

        let answer_key = String::from("TTFTTFTT");
        let k = 1;
        assert_eq!(Solution::max_consecutive_answers(answer_key, k), 5);
    }
}
