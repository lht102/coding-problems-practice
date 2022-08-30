use std::cmp::Reverse;
use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn diagonal_sort(mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let m = mat.len();
        let n = mat[0].len();
        let mut res = vec![vec![0; n]; m];
        let mut diagonals: HashMap<i32, Vec<i32>> = HashMap::new();
        for (i, arr) in mat.iter().enumerate() {
            for (j, &item) in arr.iter().enumerate() {
                diagonals.entry(i as i32 - j as i32).or_default().push(item);
            }
        }
        for val in diagonals.values_mut() {
            val.sort_unstable_by_key(|k| Reverse(*k));
        }
        for (i, arr) in res.iter_mut().enumerate() {
            for (j, item) in arr.iter_mut().enumerate() {
                *item = diagonals
                    .get_mut(&(i as i32 - j as i32))
                    .unwrap()
                    .pop()
                    .unwrap();
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
        let mat = vec![vec![3, 3, 1, 1], vec![2, 2, 1, 2], vec![1, 1, 1, 2]];
        assert_eq!(
            Solution::diagonal_sort(mat),
            vec![vec![1, 1, 1, 1], vec![1, 2, 2, 2], vec![1, 2, 3, 3]],
        );

        let mat = vec![
            vec![11, 25, 66, 1, 69, 7],
            vec![23, 55, 17, 45, 15, 52],
            vec![75, 31, 36, 44, 58, 8],
            vec![22, 27, 33, 25, 68, 4],
            vec![84, 28, 14, 11, 5, 50],
        ];
        assert_eq!(
            Solution::diagonal_sort(mat),
            vec![
                vec![5, 17, 4, 1, 52, 7],
                vec![11, 11, 25, 45, 8, 69],
                vec![14, 23, 25, 44, 58, 15],
                vec![22, 27, 31, 36, 50, 66],
                vec![84, 28, 75, 33, 55, 68],
            ]
        );
    }
}
