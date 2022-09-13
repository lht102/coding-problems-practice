struct Solution;

impl Solution {
    pub fn valid_utf8(data: Vec<i32>) -> bool {
        let mut remaining = 0;
        for d in data {
            if remaining == 0 {
                let mut prefix_one = 0;
                let mut mask = 1 << 7;
                while mask & d > 0 {
                    prefix_one += 1;
                    mask >>= 1;
                }

                if prefix_one > 4 || prefix_one == 1 {
                    return false;
                }

                if prefix_one > 1 {
                    remaining += prefix_one - 1;
                }
            } else {
                if d >> 6 != 0b10 {
                    return false;
                }
                remaining -= 1;
            }
        }
        remaining == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let data = vec![197, 130, 1];
        assert!(Solution::valid_utf8(data));

        let data = vec![235, 140, 4];
        assert!(!Solution::valid_utf8(data));
    }
}
