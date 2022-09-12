struct Solution;

impl Solution {
    pub fn bag_of_tokens_score(tokens: Vec<i32>, power: i32) -> i32 {
        if tokens.is_empty() {
            return 0;
        }
        let mut tokens = tokens;
        tokens.sort_unstable();
        let mut p = power;
        let mut res = 0;
        let mut score = 0;
        let mut left = 0;
        let mut right = tokens.len() - 1;
        while left <= right {
            if p >= tokens[left] {
                p -= tokens[left];
                score += 1;
                res = res.max(score);
                left += 1;
            } else if score > 0 {
                p += tokens[right];
                score -= 1;
                if right == 0 {
                    break;
                }
                right -= 1;
            } else {
                left += 1;
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let tokens = vec![100];
        let power = 50;
        assert_eq!(Solution::bag_of_tokens_score(tokens, power), 0);

        let tokens = vec![100, 200];
        let power = 150;
        assert_eq!(Solution::bag_of_tokens_score(tokens, power), 1);

        let tokens = vec![100, 200, 300, 400];
        let power = 250;
        assert_eq!(Solution::bag_of_tokens_score(tokens, power), 2);
    }
}
