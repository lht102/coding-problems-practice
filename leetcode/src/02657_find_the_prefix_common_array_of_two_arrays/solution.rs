struct Solution;

impl Solution {
    pub fn find_the_prefix_common_array(a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
        let mut ma: i64 = 0;
        let mut mb: i64 = 0;
        (0..a.len())
            .map(|i| {
                ma |= 1 << a[i] as i64;
                mb |= 1 << b[i] as i64;
                (ma & mb).count_ones() as i32
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let a = vec![1, 3, 2, 4];
        let b = vec![3, 1, 2, 4];
        assert_eq!(
            Solution::find_the_prefix_common_array(a, b),
            vec![0, 2, 3, 4]
        );

        let a = vec![2, 3, 1];
        let b = vec![3, 1, 2];
        assert_eq!(Solution::find_the_prefix_common_array(a, b), vec![0, 1, 3]);
    }
}
