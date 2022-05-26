struct Solution;

impl Solution {
    #![allow(non_snake_case)]
    pub fn hammingWeight(n: u32) -> i32 {
        let mut res = 0;
        for i in 0..32 {
            if n & 1 << i > 0 {
                res += 1;
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let n = 11;
        assert_eq!(Solution::hammingWeight(n), 3);

        let n = 128;
        assert_eq!(Solution::hammingWeight(n), 1);

        let n = 4294967293;
        assert_eq!(Solution::hammingWeight(n), 31);
    }
}
