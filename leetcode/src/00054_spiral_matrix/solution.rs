struct Solution;

impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let mut m = matrix.len();
        let mut n = matrix[0].len();
        let sz = m * n;
        let mut res = Vec::with_capacity(sz);
        let mut top = 0;
        let mut left = 0;
        while res.len() < sz {
            for j in left..n {
                res.push(matrix[top][j]);
            }
            top += 1;
            for row in matrix.iter().take(m).skip(top) {
                res.push(row[n - 1]);
            }
            n -= 1;
            if top < m {
                for j in (left..n).rev() {
                    res.push(matrix[m - 1][j]);
                }
                m -= 1;
            }
            if left < n {
                for i in (top..m).rev() {
                    res.push(matrix[i][left]);
                }
                left += 1;
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
        let matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        assert_eq!(
            Solution::spiral_order(matrix),
            vec![1, 2, 3, 6, 9, 8, 7, 4, 5]
        );

        let matrix = vec![vec![1, 2, 3, 4], vec![5, 6, 7, 8], vec![9, 10, 11, 12]];
        assert_eq!(
            Solution::spiral_order(matrix),
            vec![1, 2, 3, 4, 8, 12, 11, 10, 9, 5, 6, 7]
        );
    }
}
