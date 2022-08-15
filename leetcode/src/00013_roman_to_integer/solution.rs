struct Solution;

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut res = 0;
        let mut prev = 0;
        for cur in s.chars().map(|ch| Solution::roman_ch_to_int(ch)).rev() {
            if cur >= prev {
                res += cur;
            } else {
                res -= cur;
            }
            prev = cur;
        }
        res
    }

    fn roman_ch_to_int(ch: char) -> i32 {
        match ch {
            'I' => 1,
            'V' => 5,
            'X' => 10,
            'L' => 50,
            'C' => 100,
            'D' => 500,
            'M' => 1000,
            _ => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let s = String::from("III");
        assert_eq!(Solution::roman_to_int(s), 3);

        let s = String::from("LVIII");
        assert_eq!(Solution::roman_to_int(s), 58);

        let s = String::from("MCMXCIV");
        assert_eq!(Solution::roman_to_int(s), 1994);
    }
}
