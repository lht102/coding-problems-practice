struct Solution;

impl Solution {
    pub fn max_consecutive(bottom: i32, top: i32, special: Vec<i32>) -> i32 {
        let mut special = special;
        special.sort_unstable();
        let left = 0.max(special[0] - bottom);
        let right = 0.max(top - special[special.len() - 1]);
        let mut mid = 0;
        for i in 1..special.len() {
            mid = mid.max((special[i] - 1) - (special[i - 1] + 1) + 1);
        }
        left.max(right).max(mid)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let bottom = 2;
        let top = 9;
        let special = vec![4, 6];
        assert_eq!(Solution::max_consecutive(bottom, top, special), 3);

        let bottom = 6;
        let top = 8;
        let special = vec![7, 6, 8];
        assert_eq!(Solution::max_consecutive(bottom, top, special), 0);
    }
}
