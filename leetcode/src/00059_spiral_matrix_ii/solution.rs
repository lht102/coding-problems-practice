struct Solution;

impl Solution {
    pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
        let n = n as usize;
        let mut res = vec![vec![0; n]; n];
        let mut top = 0;
        let mut left = 0;
        let mut bottom = n;
        let mut right = n;
        let mut val = 0;
        let sz = n as i32 * n as i32;
        while val < sz {
            for j in left..right {
                val += 1;
                res[top][j] = val;
            }
            top += 1;
            for row in res.iter_mut().take(bottom).skip(top) {
                val += 1;
                row[right - 1] = val;
            }
            right -= 1;
            if top < bottom {
                for j in (left..right).rev() {
                    val += 1;
                    res[bottom - 1][j] = val;
                }
                bottom -= 1;
            }
            if left < right {
                for i in (top..bottom).rev() {
                    val += 1;
                    res[i][left] = val;
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
        let n = 3;
        assert_eq!(
            Solution::generate_matrix(n),
            vec![vec![1, 2, 3], vec![8, 9, 4], vec![7, 6, 5]]
        );

        let n = 1;
        assert_eq!(Solution::generate_matrix(n), vec![vec![1]],);
    }
}
