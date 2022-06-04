struct Solution;

// Definition for singly-linked list.
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
    pub fn merge_two_lists(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut dummy = ListNode::new(0);
        let mut cur = &mut dummy;

        let mut p = l1.as_ref();
        let mut q = l2.as_ref();
        while p.is_some() && q.is_some() {
            let v1 = p.unwrap();
            let v2 = q.unwrap();
            if v1.val <= v2.val {
                cur.next = Some(v1.clone());
                cur = cur.next.as_mut().unwrap();
                p = v1.next.as_ref();
            } else {
                cur.next = Some(v2.clone());
                cur = cur.next.as_mut().unwrap();
                q = v2.next.as_ref();
            }
        }
        while let Some(v) = p {
            cur.next = Some(v.clone());
            cur = cur.next.as_mut().unwrap();
            p = v.next.as_ref();
        }
        while let Some(v) = q {
            cur.next = Some(v.clone());
            cur = cur.next.as_mut().unwrap();
            q = v.next.as_ref();
        }
        dummy.next
    }
}
