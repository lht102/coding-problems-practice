struct Solution;

impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let n = num_rows as usize;
        let mut res = Vec::with_capacity(n);
        for i in 0..n {
            res.push(vec![1; i + 1]);
        }
        for i in 2..n {
            for j in 1..i {
                res[i][j] = res[i - 1][j - 1] + res[i - 1][j];
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
        let num_rows = 5;
        assert_eq!(
            Solution::generate(num_rows),
            vec![
                vec![1],
                vec![1, 1],
                vec![1, 2, 1],
                vec![1, 3, 3, 1],
                vec![1, 4, 6, 4, 1]
            ]
        );
        let num_rows = 1;
        assert_eq!(Solution::generate(num_rows), vec![vec![1]]);
    }
}
