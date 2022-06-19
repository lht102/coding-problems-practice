struct Solution;

impl Solution {
    pub fn minimum_numbers(num: i32, k: i32) -> i32 {
        if num == 0 {
            return 0;
        }
        for i in 1..=10 {
            if i * k % 10 == num % 10 && i * k <= num {
                return i;
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
        let num = 58;
        let k = 9;
        assert_eq!(Solution::minimum_numbers(num, k), 2);

        let num = 37;
        let k = 2;
        assert_eq!(Solution::minimum_numbers(num, k), -1);

        let num = 0;
        let k = 7;
        assert_eq!(Solution::minimum_numbers(num, k), 0);
    }
}
