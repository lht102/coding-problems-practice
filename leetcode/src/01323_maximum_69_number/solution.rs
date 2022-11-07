struct Solution;

impl Solution {
    pub fn maximum69_number(num: i32) -> i32 {
        let mut val = num;
        let mut digits = -1;
        let mut i = 0;
        while val > 0 {
            if val % 10 == 6 {
                digits = i as i32;
            }
            i += 1;
            val /= 10;
        }
        if digits == -1 {
            num
        } else {
            num + 3 * 10i32.pow(digits as u32)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let num = 9669;
        assert_eq!(Solution::maximum69_number(num), 9969);

        let num = 9996;
        assert_eq!(Solution::maximum69_number(num), 9999);

        let num = 9996;
        assert_eq!(Solution::maximum69_number(num), 9999);
    }
}
