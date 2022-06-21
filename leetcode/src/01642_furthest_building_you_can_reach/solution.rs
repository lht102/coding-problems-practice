use std::cmp::Reverse;
use std::collections::BinaryHeap;

struct Solution;

impl Solution {
    pub fn furthest_building(heights: Vec<i32>, bricks: i32, ladders: i32) -> i32 {
        let ladders = ladders as usize;
        let mut pq: BinaryHeap<Reverse<i32>> = BinaryHeap::new();
        let mut b = bricks;
        for i in 1..heights.len() {
            let diff = heights[i] - heights[i - 1];
            if diff > 0 {
                pq.push(Reverse(diff));
            }
            if pq.len() > ladders {
                b -= pq.pop().unwrap().0 as i32;
            }
            if b < 0 {
                return i as i32 - 1;
            }
        }
        heights.len() as i32 - 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let heights = vec![4, 2, 7, 6, 9, 14, 12];
        let bricks = 5;
        let ladders = 1;
        assert_eq!(Solution::furthest_building(heights, bricks, ladders), 4);

        let heights = vec![4, 12, 2, 7, 3, 18, 20, 3, 19];
        let bricks = 10;
        let ladders = 2;
        assert_eq!(Solution::furthest_building(heights, bricks, ladders), 7);

        let heights = vec![14, 3, 19, 3];
        let bricks = 17;
        let ladders = 0;
        assert_eq!(Solution::furthest_building(heights, bricks, ladders), 3);
    }
}
