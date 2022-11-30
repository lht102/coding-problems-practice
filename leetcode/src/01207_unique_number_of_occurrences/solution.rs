use std::collections::HashMap;
use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn unique_occurrences(arr: Vec<i32>) -> bool {
        let freq = arr
            .iter()
            .fold(HashMap::<i32, usize>::new(), |mut map, &num| {
                *map.entry(num).or_default() += 1;
                map
            })
            .into_values()
            .collect::<Vec<_>>();
        freq.len() == freq.into_iter().collect::<HashSet<_>>().len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let arr = vec![1, 2, 2, 1, 1, 3];
        assert!(Solution::unique_occurrences(arr));

        let arr = vec![1, 2];
        assert!(!Solution::unique_occurrences(arr));

        let arr = vec![-3, 0, 1, -3, 1, 1, 1, -3, 10, 0];
        assert!(Solution::unique_occurrences(arr));
    }
}
