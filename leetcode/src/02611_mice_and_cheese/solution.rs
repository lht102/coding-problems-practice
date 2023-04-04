use std::collections::BinaryHeap;
use std::iter::FromIterator;

struct Solution;

impl Solution {
    pub fn mice_and_cheese(reward1: Vec<i32>, reward2: Vec<i32>, k: i32) -> i32 {
        let mut heap = BinaryHeap::from_iter(
            reward1
                .iter()
                .zip(reward2.iter())
                .map(|(num1, num2)| num1 - num2),
        );
        reward2.iter().sum::<i32>() + (0..k).fold(0, |acc, _| acc + heap.pop().unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let reward1 = vec![1, 1, 3, 4];
        let reward2 = vec![4, 4, 1, 1];
        let k = 2;
        assert_eq!(Solution::mice_and_cheese(reward1, reward2, k), 15);

        let reward1 = vec![1, 1];
        let reward2 = vec![1, 1];
        let k = 2;
        assert_eq!(Solution::mice_and_cheese(reward1, reward2, k), 2);
    }
}
