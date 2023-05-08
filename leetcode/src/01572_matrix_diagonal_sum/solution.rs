struct Solution;

impl Solution {
    pub fn diagonal_sum(mat: Vec<Vec<i32>>) -> i32 {
        let n = mat.len();
        (0..n)
            .into_iter()
            .map(|i| mat[i][i] + mat[i][n - 1 - i])
            .sum::<i32>()
            - if n % 2 != 0 { mat[n / 2][n / 2] } else { 0 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mat = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        assert_eq!(Solution::diagonal_sum(mat), 25);

        let mat = vec![
            vec![1, 1, 1, 1],
            vec![1, 1, 1, 1],
            vec![1, 1, 1, 1],
            vec![1, 1, 1, 1],
        ];
        assert_eq!(Solution::diagonal_sum(mat), 8);

        let mat = vec![vec![5]];
        assert_eq!(Solution::diagonal_sum(mat), 5);
    }
}
