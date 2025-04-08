use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::iter::FromIterator;

struct Solution;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

impl PartialOrd for ListNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ListNode {
    fn cmp(&self, other: &Self) -> Ordering {
        self.val.cmp(&other.val)
    }
}

impl Solution {
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut min_heap = BinaryHeap::from_iter(lists.into_iter().flatten().map(Reverse));
        let mut dummy = ListNode::new(0);
        let mut cur = &mut dummy;
        while let Some(Reverse(node)) = min_heap.pop() {
            cur.next = Some(Box::new(ListNode::new(node.val)));
            cur = cur.next.as_mut()?;
            if let Some(next) = node.next {
                min_heap.push(Reverse(next));
            }
        }
        dummy.next
    }
}
