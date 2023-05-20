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
    pub fn swap_nodes(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let n = Self::length(&head);
        if n == 0 {
            return None;
        }

        let k = k as usize - 1;
        let left_idx = k.min(n - k - 1);
        let right_idx = k.max(n - k - 1);
        if left_idx == right_idx {
            return head;
        }

        let mut head = head;
        let mut p = &mut head;
        for _ in 0..left_idx {
            p = &mut p.as_mut().unwrap().next;
        }

        let mut p_node = p.take().unwrap();
        let mut p_next = p_node.next.take();
        let mut q = &mut p_next;
        for _ in left_idx + 1..right_idx {
            q = &mut q.as_mut().unwrap().next;
        }

        let mut q_node = q.take().unwrap();
        p_node.next = q_node.next.take();
        *q = Some(p_node);
        q_node.next = p_next;
        *p = Some(q_node);

        head
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let head = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: Some(Box::new(ListNode {
                        val: 4,
                        next: Some(Box::new(ListNode { val: 5, next: None })),
                    })),
                })),
            })),
        }));
        let k = 2;
        assert_eq!(
            Solution::swap_nodes(head, k),
            Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 4,
                    next: Some(Box::new(ListNode {
                        val: 3,
                        next: Some(Box::new(ListNode {
                            val: 2,
                            next: Some(Box::new(ListNode { val: 5, next: None })),
                        })),
                    })),
                })),
            }))
        );
    }
}
