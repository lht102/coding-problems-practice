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

impl Solution {
    pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if let Some(mut p) = head {
            if let Some(mut q) = p.next {
                p.next = Self::swap_pairs(q.next);
                q.next = Some(p);
                Some(q)
            } else {
                Some(p)
            }
        } else {
            None
        }
    }
}
