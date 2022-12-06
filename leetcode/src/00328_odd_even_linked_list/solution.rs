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
    pub fn odd_even_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = head;
        let mut dummy_odd = ListNode::new(0);
        let mut dummy_even = ListNode::new(0);
        let mut cur_odd = &mut dummy_odd;
        let mut cur_even = &mut dummy_even;
        let mut is_odd = true;
        while let Some(mut node) = head {
            head = node.next.take();
            if is_odd {
                cur_odd.next = Some(node);
                cur_odd = cur_odd.next.as_mut().unwrap();
            } else {
                cur_even.next = Some(node);
                cur_even = cur_even.next.as_mut().unwrap();
            }
            is_odd = !is_odd;
        }
        cur_odd.next = dummy_even.next;
        dummy_odd.next
    }
}
