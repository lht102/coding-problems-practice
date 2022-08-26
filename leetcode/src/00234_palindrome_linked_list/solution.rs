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
    pub fn is_palindrome(mut head: Option<Box<ListNode>>) -> bool {
        let mut slow = &head;
        let mut fast = &head;
        let mut n = 0;
        while let Some(node) = fast {
            fast = &node.next;
            if let Some(node) = fast {
                fast = &node.next;
            }
            if let Some(node) = slow {
                slow = &node.next;
                n += 1;
            }
        }

        let mut middle = &mut head;
        while n > 1 {
            if let Some(node) = middle {
                middle = &mut node.next;
                n -= 1;
            }
        }

        if let Some(node) = middle {
            let reversed = reverse(node.next.take());

            let mut p1 = &head;
            let mut p2 = &reversed;
            while let (Some(n1), Some(n2)) = (p1, p2) {
                if n1.val != n2.val {
                    return false;
                }
                p1 = &n1.next;
                p2 = &n2.next;
            }
        }

        true
    }
}

fn reverse(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let (mut prev, mut cur) = (None, head);
    while let Some(mut node) = cur {
        cur = node.next;
        node.next = prev;
        prev = Some(node);
    }
    prev
}
