use std::mem::swap;

struct Solution;

impl Solution {
    pub fn gcd_of_strings(str1: String, str2: String) -> String {
        if format!("{str1}{str2}") != format!("{str2}{str1}") {
            return String::new();
        }
        let k = Solution::gcd(str1.len(), str2.len());
        str1[..k].to_owned()
    }

    fn gcd(a: usize, b: usize) -> usize {
        let mut a = a;
        let mut b = b;
        while b != 0 {
            a %= b;
            swap(&mut a, &mut b);
        }
        a
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let str1 = String::from("ABCABC");
        let str2 = String::from("ABC");
        assert_eq!(Solution::gcd_of_strings(str1, str2), String::from("ABC"));

        let str1 = String::from("ABABAB");
        let str2 = String::from("ABAB");
        assert_eq!(Solution::gcd_of_strings(str1, str2), String::from("AB"));

        let str1 = String::from("LEET");
        let str2 = String::from("CODE");
        assert_eq!(Solution::gcd_of_strings(str1, str2), String::from(""));
    }
}
