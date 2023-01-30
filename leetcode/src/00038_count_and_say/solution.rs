struct Solution;

impl Solution {
    pub fn count_and_say(n: i32) -> String {
        if n == 1 {
            return String::from("1");
        }
        Solution::solve(Solution::count_and_say(n - 1).chars().collect())
    }

    fn solve(s: Vec<char>) -> String {
        let mut res = String::new();
        let mut prev = s[0];
        let mut cnt = 1;
        for &ch in s.iter().skip(1) {
            if ch == prev {
                cnt += 1;
            } else {
                res.push_str(&format!("{cnt}{prev}"));
                prev = ch;
                cnt = 1;
            }
        }
        res.push_str(&format!("{cnt}{prev}"));
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let n = 1;
        assert_eq!(Solution::count_and_say(n), String::from("1"));

        let n = 4;
        assert_eq!(Solution::count_and_say(n), String::from("1211"));
    }
}
