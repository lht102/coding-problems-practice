struct Solution;

impl Solution {
    pub fn count_texts(pressed_keys: String) -> i32 {
        let m = 1_000_000_007;
        let pressed_keys: Vec<char> = pressed_keys.chars().collect();
        let n = pressed_keys.len();
        let mut dp = vec![0; n + 1];
        dp[0] = 1;
        for i in 1..=n {
            let ch = pressed_keys[i - 1];
            dp[i] = dp[i - 1];
            if i >= 2 && ch == pressed_keys[i - 2] {
                dp[i] += dp[i - 2];
                dp[i] %= m;
                if i >= 3 && ch == pressed_keys[i - 3] {
                    dp[i] += dp[i - 3];
                    dp[i] %= m;
                    if i >= 4
                        && ch == pressed_keys[i - 4]
                        && (pressed_keys[i - 1] == '7' || pressed_keys[i - 1] == '9')
                    {
                        dp[i] += dp[i - 4];
                        dp[i] %= m;
                    }
                }
            }
        }
        dp[n]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let pressed_keys = String::from("22233");
        assert_eq!(Solution::count_texts(pressed_keys), 8);

        let pressed_keys = String::from("222222222222222222222222222222222222");
        assert_eq!(Solution::count_texts(pressed_keys), 82876089);
    }
}
