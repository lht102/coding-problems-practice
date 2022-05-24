struct Solution;

impl Solution {
    pub fn longest_valid_parentheses(s: String) -> i32 {
        let mut dp = vec![-1; s.len()];
        let s = s.chars().collect::<Vec<_>>();
        for i in 0..s.len() {
            Solution::helper(&s, i as i32, &mut dp);
        }
        *dp.iter().max().unwrap_or(&0)
    }

    fn helper(s: &[char], i: i32, dp: &mut Vec<i32>) -> i32 {
        if i < 0 {
            return 0;
        }
        let idx = i as usize;
        if dp[idx] != -1 {
            return dp[idx];
        }
        dp[idx] = if s[idx] == '(' {
            0
        } else if i <= 1 {
            if s[0] == '(' && s[1] == ')' {
                2
            } else {
                0
            }
        } else if s[idx - 1] == '(' {
            Solution::helper(s, i - 2, dp) + 2
        } else {
            let prev = i - Solution::helper(s, i - 1, dp) - 1;
            if prev < 0 {
                0
            } else if s[prev as usize] == '(' {
                Solution::helper(s, i - 1, dp)
                    + Solution::helper(s, i - Solution::helper(s, i - 1, dp) - 2, dp)
                    + 2
            } else {
                0
            }
        };
        dp[idx]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let s = String::from("(()");
        assert_eq!(Solution::longest_valid_parentheses(s), 2);

        let s = String::from(")()())");
        assert_eq!(Solution::longest_valid_parentheses(s), 4);

        let s = String::from("");
        assert_eq!(Solution::longest_valid_parentheses(s), 0);
    }
}
