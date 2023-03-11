use std::cell::RefCell;
use std::rc::Rc;

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

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

impl Solution {
    pub fn sorted_list_to_bst(head: Option<Box<ListNode>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut n = 0;
        let mut cur = &head;
        while let Some(node) = cur {
            n += 1;
            cur = &node.next;
        }

        let mut head = head;
        Self::dfs(&mut head, n)
    }

    fn dfs(head: &mut Option<Box<ListNode>>, n: usize) -> Option<Rc<RefCell<TreeNode>>> {
        if n == 0 {
            return None;
        }
        let m = n / 2;
        let left_node = Self::dfs(head, m);
        if let Some(node) = head {
            let mut root = TreeNode::new(node.val);
            *head = node.next.take();
            root.left = left_node;
            root.right = Self::dfs(head, n - m - 1);
            Some(Rc::new(RefCell::new(root)))
        } else {
            None
        }
    }
}
