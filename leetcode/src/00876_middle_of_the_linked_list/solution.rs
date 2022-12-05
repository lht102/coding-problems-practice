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
    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut slow = &head;
        let mut fast = &head;
        while let Some(node) = fast {
            fast = &node.next;
            if let Some(node) = fast {
                fast = &node.next;
            } else {
                break;
            }
            if let Some(node) = slow {
                slow = &node.next;
            }
        }
        slow.clone()
    }
}
