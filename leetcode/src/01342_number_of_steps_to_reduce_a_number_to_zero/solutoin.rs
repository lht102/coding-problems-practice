struct Solution;

impl Solution {
    pub fn number_of_steps(num: i32) -> i32 {
        let mut res = 0;
        let mut num = num;
        while num > 0 {
            res += if num == 1 || num & 1 == 0 { 1 } else { 2 };
            num >>= 1;
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let num = 14;
        assert_eq!(Solution::number_of_steps(num), 6);

        let num = 8;
        assert_eq!(Solution::number_of_steps(num), 4);

        let num = 123;
        assert_eq!(Solution::number_of_steps(num), 12);
    }
}
