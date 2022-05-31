struct Solution;

impl Solution {
    pub fn has_all_codes(s: String, k: i32) -> bool {
        let k = k as usize;
        let mut n = 1 << k;
        let mut vi = vec![false; n];
        let mask = n - 1;
        let mut hash = 0;
        for (i, ch) in s.bytes().enumerate() {
            hash = (hash << 1 & mask) | (ch - b'0') as usize;
            if i + 1 >= k && !vi[hash] {
                vi[hash] = true;
                n -= 1;
                if n == 0 {
                    return true;
                }
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let s = String::from("00110110");
        let k = 2;
        assert!(Solution::has_all_codes(s, k));

        let s = String::from("0110");
        let k = 1;
        assert!(Solution::has_all_codes(s, k));

        let s = String::from("0110");
        let k = 2;
        assert!(!Solution::has_all_codes(s, k));
    }
}
