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
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let m = Solution::length(&head);
        let mut dummy = Some(Box::new(ListNode { val: 0, next: head }));
        let mut p = &mut dummy;
        for _ in 0..m - n as usize {
            p = &mut p.as_mut().unwrap().next;
        }
        let removed = p.as_mut().unwrap().next.take();
        if let Some(q) = removed {
            p.as_mut().unwrap().next = q.next;
        }
        dummy.unwrap().next
    }

    fn length(head: &Option<Box<ListNode>>) -> usize {
        let mut res = 0;
        let mut p = head;
        while p.is_some() {
            res += 1;
            p = &p.as_ref().unwrap().next;
        }
        res
    }
}
