struct Solution;

impl Solution {
    pub fn transpose(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut res = vec![vec![0; matrix.len()]; matrix[0].len()];
        for (i, arr1) in matrix.iter().enumerate() {
            for (j, arr2) in res.iter_mut().enumerate() {
                arr2[i] = arr1[j];
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
            Solution::transpose(matrix),
            vec![vec![1, 4, 7], vec![2, 5, 8], vec![3, 6, 9]]
        );

        let matrix = vec![vec![1, 2, 3], vec![4, 5, 6]];
        assert_eq!(
            Solution::transpose(matrix),
            vec![vec![1, 4], vec![2, 5], vec![3, 6],],
        );
    }
}
