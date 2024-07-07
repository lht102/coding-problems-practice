struct Solution;

impl Solution {
    pub fn three_consecutive_odds(arr: Vec<i32>) -> bool {
        arr.windows(3).any(|x| x[0] & x[1] & x[2] & 1 == 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let arr = vec![2, 6, 4, 1];
        assert!(!Solution::three_consecutive_odds(arr));

        let arr = vec![1, 2, 34, 3, 4, 5, 7, 23, 12];
        assert!(Solution::three_consecutive_odds(arr));
    }
}
