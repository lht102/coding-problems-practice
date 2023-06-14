use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn equal_pairs(grid: Vec<Vec<i32>>) -> i32 {
        let mut freq = HashMap::new();
        for row in &grid {
            *freq.entry(row.clone()).or_default() += 1;
        }
        let mut res = 0;
        let n = grid.len();
        for j in 0..n {
            let mut col = Vec::with_capacity(n);
            for row in &grid {
                col.push(row[j]);
            }
            res += freq.get(&col).unwrap_or(&0);
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let grid = vec![vec![3, 2, 1], vec![1, 7, 6], vec![2, 7, 7]];
        assert_eq!(Solution::equal_pairs(grid), 1);

        let grid = vec![
            vec![3, 1, 2, 2],
            vec![1, 4, 4, 5],
            vec![2, 4, 2, 2],
            vec![2, 4, 2, 2],
        ];
        assert_eq!(Solution::equal_pairs(grid), 3);

        let grid = vec![
            vec![6, 7, 2, 2],
            vec![8, 9, 0, 4],
            vec![2, 4, 2, 2],
            vec![2, 4, 2, 2],
        ];
        assert_eq!(Solution::equal_pairs(grid), 2);
    }
}
