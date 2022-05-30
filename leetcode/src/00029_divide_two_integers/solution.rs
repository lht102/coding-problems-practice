struct Solution;

impl Solution {
    pub fn divide(dividend: i32, divisor: i32) -> i32 {
        if dividend == i32::MIN && divisor == -1 {
            return i32::MAX;
        }
        if dividend == i32::MIN && divisor == 1 {
            return i32::MIN;
        }
        let mut res = 0;
        let mut a = dividend.abs();
        let b = divisor.abs();
        while a - b >= 0 {
            let mut shift = 0;
            while a - (b << (shift + 1)) >= 0 {
                shift += 1;
            }
            res += 1 << shift;
            a -= b << shift;
        }
        if (dividend > 0) == (divisor > 0) {
            res
        } else {
            -res
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let dividend = 10;
        let divisor = 3;
        assert_eq!(Solution::divide(dividend, divisor), 3);

        let dividend = 7;
        let divisor = -3;
        assert_eq!(Solution::divide(dividend, divisor), -2);

        let dividend = -2147483648;
        let divisor = 1;
        assert_eq!(Solution::divide(dividend, divisor), -2147483648);
    }
}
