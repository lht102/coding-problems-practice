struct Solution;

impl Solution {
    pub fn reordered_power_of2(n: i32) -> bool {
        let arr = Solution::counter(n);
        for i in 0..32 {
            if arr == Solution::counter(1 << i) {
                return true;
            }
        }
        false
    }

    fn counter(n: i32) -> [i32; 10] {
        let mut res = [0; 10];
        let mut tmp = n;
        while tmp > 0 {
            res[(tmp % 10) as usize] += 1;
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
        let n = 1;
        assert!(Solution::reordered_power_of2(n));

        let n = 10;
        assert!(!Solution::reordered_power_of2(n));
    }
}
