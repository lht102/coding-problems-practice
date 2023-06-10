struct Solution;

impl Solution {
    pub fn min_flips(a: i32, b: i32, c: i32) -> i32 {
        let mut res = 0;
        for i in 0..32 {
            let a_mask = (1 << i) & a;
            let b_mask = (1 << i) & b;
            let c_mask = (1 << i) & c;
            if c_mask == 0 {
                if (a_mask & b_mask) > 0 {
                    res += 2;
                } else if (a_mask | b_mask) > 0 {
                    res += 1;
                }
            } else if a_mask == 0 && b_mask == 0 {
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
        let a = 2;
        let b = 6;
        let c = 5;
        assert_eq!(Solution::min_flips(a, b, c), 3);

        let a = 4;
        let b = 2;
        let c = 7;
        assert_eq!(Solution::min_flips(a, b, c), 1);

        let a = 8;
        let b = 3;
        let c = 5;
        assert_eq!(Solution::min_flips(a, b, c), 3);
    }
}
