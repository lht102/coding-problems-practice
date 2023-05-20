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
    pub fn pair_sum(head: Option<Box<ListNode>>) -> i32 {
        let mut slow = &head;
        let mut fast = &head;
        while let Some(node) = fast {
            if let Some(node) = &node.next {
                fast = &node.next;
            }
            if let Some(node) = slow {
                slow = &node.next;
            }
        }

        let mut rhead = Self::reverse(slow.clone());
        let mut head = head;
        let mut res = 0;
        while let Some(rhead_node) = rhead {
            let head_node = head.unwrap();
            res = res.max(head_node.val + rhead_node.val);
            head = head_node.next;
            rhead = rhead_node.next;
        }
        res
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let head = Some(Box::new(ListNode {
            val: 5,
            next: Some(Box::new(ListNode {
                val: 4,
                next: Some(Box::new(ListNode {
                    val: 2,
                    next: Some(Box::new(ListNode { val: 1, next: None })),
                })),
            })),
        }));
        assert_eq!(Solution::pair_sum(head), 6);

        let head = Some(Box::new(ListNode {
            val: 4,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 2,
                    next: Some(Box::new(ListNode { val: 3, next: None })),
                })),
            })),
        }));
        assert_eq!(Solution::pair_sum(head), 7);

        let head = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 100000,
                next: None,
            })),
        }));
        assert_eq!(Solution::pair_sum(head), 100001);
    }
}
