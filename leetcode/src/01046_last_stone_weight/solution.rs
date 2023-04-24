use std::collections::BinaryHeap;

struct Solution;

impl Solution {
    pub fn last_stone_weight(stones: Vec<i32>) -> i32 {
        let mut pq = BinaryHeap::from(stones);
        while let Some(y) = pq.pop() {
            if let Some(x) = pq.pop() {
                if x != y {
                    pq.push(y - x);
                }
            } else {
                return y;
            }
        }
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let stones = vec![2, 7, 4, 1, 8, 1];
        assert_eq!(Solution::last_stone_weight(stones), 1);

        let stones = vec![1];
        assert_eq!(Solution::last_stone_weight(stones), 1);
    }
}
