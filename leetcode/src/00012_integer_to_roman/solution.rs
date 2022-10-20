struct Solution;

impl Solution {
    pub fn int_to_roman(num: i32) -> String {
        let roman_lookup = [
            ("M", 1000),
            ("CM", 900),
            ("D", 500),
            ("CD", 400),
            ("C", 100),
            ("XC", 90),
            ("L", 50),
            ("XL", 40),
            ("X", 10),
            ("IX", 9),
            ("V", 5),
            ("IV", 4),
            ("I", 1),
        ];
        let mut res = String::new();
        let mut n = num;
        let mut i = 0;
        while n > 0 {
            while n >= roman_lookup[i].1 {
                n -= roman_lookup[i].1;
                res.push_str(roman_lookup[i].0);
            }
            i += 1;
        }
        res
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let num = 3;
        assert_eq!(Solution::int_to_roman(num), String::from("III"));

        let num = 58;
        assert_eq!(Solution::int_to_roman(num), String::from("LVIII"));

        let num = 1994;
        assert_eq!(Solution::int_to_roman(num), String::from("MCMXCIV"));
    }
}
