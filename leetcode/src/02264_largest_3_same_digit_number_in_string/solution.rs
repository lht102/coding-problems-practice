struct Solution;

impl Solution {
    pub fn largest_good_integer(num: String) -> String {
        let mut res = '\0';
        let num: Vec<char> = num.chars().collect();
        for i in 2..num.len() {
            if num[i - 2] == num[i - 1] && num[i - 1] == num[i] && res < num[i] {
                res = num[i];
            }
        }
        if res == '\0' {
            String::new()
        } else {
            std::iter::repeat(res).take(3).collect()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let num = String::from("6777133339");
        assert_eq!(Solution::largest_good_integer(num), String::from("777"));

        let num = String::from("2300019");
        assert_eq!(Solution::largest_good_integer(num), String::from("000"));

        let num = String::from("42352338");
        assert_eq!(Solution::largest_good_integer(num), String::from(""));
    }
}
