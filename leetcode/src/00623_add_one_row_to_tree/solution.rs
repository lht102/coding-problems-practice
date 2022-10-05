use std::cell::RefCell;
use std::rc::Rc;

struct Solution;

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
    pub fn add_one_row(
        root: Option<Rc<RefCell<TreeNode>>>,
        val: i32,
        depth: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if depth == 1 {
            Some(Rc::new(RefCell::new(TreeNode {
                val,
                left: root,
                right: None,
            })))
        } else {
            Solution::dfs(&root, val, depth);
            root
        }
    }

    fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, val: i32, depth: i32) {
        if let Some(r) = root {
            if depth > 2 {
                let r = r.borrow();
                Solution::dfs(&r.left, val, depth - 1);
                Solution::dfs(&r.right, val, depth - 1);
            } else {
                let mut r = r.borrow_mut();
                r.left = Some(Rc::new(RefCell::new(TreeNode {
                    val,
                    left: r.left.take(),
                    right: None,
                })));
                r.right = Some(Rc::new(RefCell::new(TreeNode {
                    val,
                    left: None,
                    right: r.right.take(),
                })));
            }
        }
    }
}
