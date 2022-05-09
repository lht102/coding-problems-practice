struct Solution;

impl Solution {
    pub fn minimum_removal(beans: Vec<i32>) -> i64 {
        let n = beans.len();
        let mut beans = beans;
        beans.sort_unstable();
        let sum: i64 = beans.iter().map(|&num| num as i64).sum();
        let mut res = i64::MAX;
        for (i, &num) in beans.iter().enumerate() {
            res = res.min(sum - (n - i) as i64 * num as i64);
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let beans = vec![4, 1, 6, 5];
        assert_eq!(Solution::minimum_removal(beans), 4);

        let beans = vec![2, 10, 3, 2];
        assert_eq!(Solution::minimum_removal(beans), 7);
    }
}
