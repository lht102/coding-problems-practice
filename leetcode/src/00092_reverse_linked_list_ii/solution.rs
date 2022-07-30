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
    pub fn reverse_between(
        head: Option<Box<ListNode>>,
        left: i32,
        right: i32,
    ) -> Option<Box<ListNode>> {
        let mut dummy = Some(Box::new(ListNode { val: 0, next: head }));
        let mut prev = &mut dummy;
        for _ in 1..left {
            prev = &mut prev.as_mut().unwrap().next;
        }
        let mut cur = prev.as_mut().unwrap().next.take();
        for _ in left..right {
            let mut next = cur.as_mut().unwrap().next.take();
            cur.as_mut().unwrap().next = next.as_mut().unwrap().next.take();
            next.as_mut().unwrap().next = prev.as_mut().unwrap().next.take();
            prev.as_mut().unwrap().next = next;
        }
        while prev.as_ref().unwrap().next.is_some() {
            prev = &mut prev.as_mut().unwrap().next;
        }
        prev.as_mut().unwrap().next = cur;
        dummy.unwrap().next
    }
}
