use std::cmp::Reverse;
use std::collections::BinaryHeap;

struct Solution;

#[derive(Default)]
struct MedianFinder {
    max_heap: BinaryHeap<i32>,
    min_heap: BinaryHeap<Reverse<i32>>,
}

impl MedianFinder {
    fn new() -> Self {
        Default::default()
    }

    fn add_num(&mut self, num: i32) {
        self.max_heap.push(num);
        self.min_heap.push(Reverse(self.max_heap.pop().unwrap()));
        if self.min_heap.len() > self.max_heap.len() {
            self.max_heap.push(self.min_heap.pop().unwrap().0);
        }
    }

    fn find_median(&self) -> f64 {
        if self.max_heap.len() > self.min_heap.len() {
            *self.max_heap.peek().unwrap() as _
        } else {
            (self.max_heap.peek().unwrap() + self.min_heap.peek().unwrap().0) as f64 / 2.0
        }
    }
}
