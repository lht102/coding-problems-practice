struct Solution;

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        if needle.len() > haystack.len() {
            return -1;
        }

        let p = 31;
        let m = 1_000_000_009;
        let h_len = haystack.len();
        let n_len = needle.len();
        let haystack = haystack.as_bytes();
        let needle = needle.as_bytes();

        let mut p_pow = vec![0; h_len.max(n_len)];
        p_pow[0] = 1;
        for i in 1..p_pow.len() {
            p_pow[i] = p_pow[i - 1] * p % m;
        }

        let mut h_fingerprint = vec![0; h_len + 1];
        for i in 0..h_len {
            h_fingerprint[i + 1] =
                (h_fingerprint[i] + (haystack[i] - b'a' + 1) as i64 * p_pow[i]) % m;
        }
        let mut n_fingerprint = 0;
        for i in 0..n_len {
            n_fingerprint = (n_fingerprint + ((needle[i] - b'a' + 1) as i64) * p_pow[i]) % m;
        }

        for i in 0..h_len - n_len + 1 {
            let fingerprint = (h_fingerprint[i + n_len] + m - h_fingerprint[i]) % m;
            if fingerprint == n_fingerprint * p_pow[i] % m {
                return i as _;
            }
        }

        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let haystack = String::from("sadbutsad");
        let needle = String::from("sad");
        assert_eq!(Solution::str_str(haystack, needle), 0);

        let haystack = String::from("leetcode");
        let needle = String::from("leeto");
        assert_eq!(Solution::str_str(haystack, needle), -1);

        let haystack = String::from("abb");
        let needle = String::from("abaaa");
        assert_eq!(Solution::str_str(haystack, needle), -1);
    }
}
