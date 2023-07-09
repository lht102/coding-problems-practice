struct Solution;

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let (mut c1, mut c2) = (0, 0);
        for &num in &nums {
            c2 ^= c1 & num;
            c1 ^= num;
            let mask = !(c1 & c2);
            c1 &= mask;
            c2 &= mask;
        }
        c1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let nums = vec![2, 2, 3, 2];
        assert_eq!(Solution::single_number(nums), 3);

        let nums = vec![0, 1, 0, 1, 0, 1, 99];
        assert_eq!(Solution::single_number(nums), 99);
    }
}
