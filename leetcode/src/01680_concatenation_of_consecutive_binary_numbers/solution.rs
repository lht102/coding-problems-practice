struct Solution;

impl Solution {
    pub fn concatenated_binary(n: i32) -> i32 {
        let mut res: i64 = 0;
        let mut num_of_bits = 0;
        for i in 1..=n as i64 {
            if i & (i - 1) == 0 {
                num_of_bits += 1;
            }
            res = ((res << num_of_bits) | i) % 1_000_000_007;
        }
        res as _
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let n = 3;
        assert_eq!(Solution::concatenated_binary(n), 27);

        let n = 12;
        assert_eq!(Solution::concatenated_binary(n), 505379714);
    }
}
