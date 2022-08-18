use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::iter::FromIterator;

struct Solution;

impl Solution {
    pub fn min_set_size(arr: Vec<i32>) -> i32 {
        let half = arr.len() / 2;
        let mut max_heap = BinaryHeap::from_iter(
            arr.into_iter()
                .fold(HashMap::<i32, usize>::new(), |mut map, num| {
                    *map.entry(num).or_default() += 1;
                    map
                })
                .into_values(),
        );
        let mut removed = 0;
        let mut res = 0;
        while let Some(cnt) = max_heap.pop() {
            removed += cnt;
            res += 1;
            if removed >= half {
                break;
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
        let arr = vec![3, 3, 3, 3, 5, 5, 5, 2, 2, 7];
        assert_eq!(Solution::min_set_size(arr), 2);

        let arr = vec![7, 7, 7, 7, 7, 7];
        assert_eq!(Solution::min_set_size(arr), 1);
    }
}
