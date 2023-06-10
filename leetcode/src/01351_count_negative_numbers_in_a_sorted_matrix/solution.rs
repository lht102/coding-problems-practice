struct Solution;

impl Solution {
    pub fn count_negatives(grid: Vec<Vec<i32>>) -> i32 {
        grid.iter()
            .map(|row| {
                let i = row.partition_point(|&x| x > -1);
                (row.len() - i) as i32
            })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let grid = vec![
            vec![4, 3, 2, -1],
            vec![3, 2, 1, -1],
            vec![1, 1, -1, -2],
            vec![-1, -1, -2, -3],
        ];
        assert_eq!(Solution::count_negatives(grid), 8);

        let grid = vec![vec![3, 2], vec![1, 0]];
        assert_eq!(Solution::count_negatives(grid), 0);
    }
}
