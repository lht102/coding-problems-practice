struct Solution;

impl Solution {
    pub fn sum_of_number_and_reverse(num: i32) -> bool {
        (0..=num).any(|i| i + Solution::reverse_i32(i) == num)
    }

    fn reverse_i32(n: i32) -> i32 {
        let mut res = 0;
        let mut tmp = n;
        while tmp != 0 {
            res = res * 10 + tmp % 10;
            tmp /= 10;
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let num = 443;
        assert!(Solution::sum_of_number_and_reverse(num));

        let num = 63;
        assert!(!Solution::sum_of_number_and_reverse(num));

        let num = 181;
        assert!(Solution::sum_of_number_and_reverse(num));
    }
}
