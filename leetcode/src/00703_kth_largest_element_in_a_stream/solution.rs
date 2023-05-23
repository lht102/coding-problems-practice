use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::iter::FromIterator;

struct Solution;

struct KthLargest {
    k: usize,
    pq: BinaryHeap<Reverse<i32>>,
}

impl KthLargest {
    fn new(k: i32, nums: Vec<i32>) -> Self {
        Self {
            k: k as usize,
            pq: BinaryHeap::from_iter(nums.into_iter().map(Reverse)),
        }
    }

    fn add(&mut self, val: i32) -> i32 {
        self.pq.push(Reverse(val));
        while self.pq.len() > self.k {
            self.pq.pop();
        }
        self.pq.peek().unwrap().0
    }
}
