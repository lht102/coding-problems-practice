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
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let (mut prev, mut cur) = (None, head);
        while let Some(mut node) = cur {
            cur = node.next;
            node.next = prev;
            prev = Some(node)
        }
        prev
    }

    fn solve(head: Option<Box<ListNode>>, prev: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        match head {
            Some(mut node) => {
                let cur = node.next.take();
                node.next = prev;
                Solution::solve(cur, Some(node))
            }
            None => prev,
        }
    }
}
