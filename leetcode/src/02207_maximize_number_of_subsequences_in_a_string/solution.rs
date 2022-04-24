struct Solution;

impl Solution {
    pub fn maximum_subsequence_count(text: String, pattern: String) -> i64 {
        let t: Vec<_> = text.chars().collect();
        let p: Vec<_> = pattern.chars().collect();
        let mut res = 0;
        let (mut cnt1, mut cnt2) = (0, 0);
        for &ch in &t {
            if ch == p[1] {
                res += cnt1;
                cnt2 += 1;
            }
            if ch == p[0] {
                cnt1 += 1;
            }
        }
        res + cnt1.max(cnt2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let text = String::from("abdcdbc");
        let pattern = String::from("ac");
        assert_eq!(Solution::maximum_subsequence_count(text, pattern), 4);

        let text = String::from("aabb");
        let pattern = String::from("ab");
        assert_eq!(Solution::maximum_subsequence_count(text, pattern), 6);

        let text = String::from("aabaa");
        let pattern = String::from("aa");
        assert_eq!(Solution::maximum_subsequence_count(text, pattern), 10);
    }
}
