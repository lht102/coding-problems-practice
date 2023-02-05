struct Solution;

impl Solution {
    pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
        if p.len() > s.len() {
            return vec![];
        }
        let s = s.as_bytes();
        let p = p.as_bytes();
        let get_index = |x: u8| (x - b'a') as usize;
        let fingerprint = p.iter().fold(vec![0; 26], |mut acc, &x| {
            acc[get_index(x)] += 1;
            acc
        });
        let mut rolling_fingerprint = s[..p.len()].iter().fold(vec![0; 26], |mut acc, &x| {
            acc[get_index(x)] += 1;
            acc
        });
        let mut res = vec![];
        if fingerprint == rolling_fingerprint {
            res.push(0);
        }
        for end in p.len()..s.len() {
            let start = end - p.len();
            rolling_fingerprint[get_index(s[start])] -= 1;
            rolling_fingerprint[get_index(s[end])] += 1;
            if fingerprint == rolling_fingerprint {
                res.push(start as i32 + 1);
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
        let s = String::from("cbaebabacd");
        let p = String::from("abc");
        assert_eq!(Solution::find_anagrams(s, p), vec![0, 6]);

        let s = String::from("abab");
        let p = String::from("ab");
        assert_eq!(Solution::find_anagrams(s, p), vec![0, 1, 2]);
    }
}
