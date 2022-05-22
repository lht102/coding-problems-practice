use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::iter::FromIterator;

struct Solution;

impl Solution {
    pub fn maximum_bags(capacity: Vec<i32>, rocks: Vec<i32>, additional_rocks: i32) -> i32 {
        let mut hp = BinaryHeap::from_iter(
            capacity
                .iter()
                .zip(rocks.iter())
                .map(|(&a, &b)| a - b)
                .fold(HashMap::<Reverse<i32>, usize>::new(), |mut map, num| {
                    *map.entry(Reverse(num)).or_default() += 1;
                    map
                }),
        );
        let mut remaining = additional_rocks;
        let mut res = 0;
        while remaining > 0 {
            if let Some((Reverse(num), cnt)) = hp.pop() {
                let fill = remaining.min(num * cnt as i32);
                remaining -= fill;
                if num != 0 {
                    res += fill / num;
                } else {
                    res += cnt as i32;
                }
            } else {
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
        let capacity = vec![2, 3, 4, 5];
        let rocks = vec![1, 2, 4, 4];
        let additional_rocks = 2;
        assert_eq!(Solution::maximum_bags(capacity, rocks, additional_rocks), 3);

        let capacity = vec![10, 2, 2];
        let rocks = vec![2, 2, 0];
        let additional_rocks = 100;
        assert_eq!(Solution::maximum_bags(capacity, rocks, additional_rocks), 3);
    }
}
