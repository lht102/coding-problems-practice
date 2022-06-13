struct Solution;

impl Solution {
    pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
        let mut triangle = triangle;
        let n = triangle.len();
        for i in (0..n).rev().skip(1) {
            for j in 0..triangle[i].len() {
                triangle[i][j] += triangle[i + 1][j].min(triangle[i + 1][j + 1]);
            }
        }
        triangle[0][0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let triangle = vec![vec![2], vec![3, 4], vec![6, 5, 7], vec![4, 1, 8, 3]];
        assert_eq!(Solution::minimum_total(triangle), 11);

        let triangle = vec![vec![-10]];
        assert_eq!(Solution::minimum_total(triangle), -10);
    }
}
