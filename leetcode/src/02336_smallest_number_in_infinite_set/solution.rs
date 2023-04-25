use std::collections::BTreeSet;

struct Solution;

struct SmallestInfiniteSet {
    set: BTreeSet<i32>,
    smallest: i32,
}

impl SmallestInfiniteSet {
    fn new() -> Self {
        Self {
            set: Default::default(),
            smallest: 1,
        }
    }

    fn pop_smallest(&mut self) -> i32 {
        if let Some(&val) = self.set.iter().next() {
            self.set.remove(&val);
            val
        } else {
            self.smallest += 1;
            self.smallest - 1
        }
    }

    fn add_back(&mut self, num: i32) {
        if num < self.smallest {
            self.set.insert(num);
        }
    }
}
