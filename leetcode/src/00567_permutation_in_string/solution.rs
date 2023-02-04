struct Solution;

impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        if s1.len() > s2.len() {
            return false;
        }
        let s1 = s1.as_bytes();
        let s2 = s2.as_bytes();
        let get_index = |x: u8| (x - b'a') as usize;
        let fingerprint = s1.iter().fold(vec![0; 26], |mut acc, &x| {
            acc[get_index(x)] += 1;
            acc
        });

        let mut rolling_fingerprint = s2[..s1.len()].iter().fold(vec![0; 26], |mut acc, &x| {
            acc[get_index(x)] += 1;
            acc
        });
        if fingerprint == rolling_fingerprint {
            return true;
        }
        for end in s1.len()..s2.len() {
            rolling_fingerprint[get_index(s2[end - s1.len()])] -= 1;
            rolling_fingerprint[get_index(s2[end])] += 1;
            if fingerprint == rolling_fingerprint {
                return true;
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
        let s1 = String::from("ab");
        let s2 = String::from("eidbaooo");
        assert!(Solution::check_inclusion(s1, s2));

        let s1 = String::from("ab");
        let s2 = String::from("eidboaoo");
        assert!(!Solution::check_inclusion(s1, s2));
    }
}
