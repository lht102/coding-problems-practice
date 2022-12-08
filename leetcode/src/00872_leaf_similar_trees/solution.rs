use std::cell::RefCell;
use std::rc::Rc;

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

struct Solution;

impl Solution {
    pub fn leaf_similar(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        let mut leaf1 = Vec::new();
        Solution::dfs(&root1, &mut leaf1);
        let mut leaf2 = Vec::new();
        Solution::dfs(&root2, &mut leaf2);
        leaf1 == leaf2
    }

    fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, res: &mut Vec<i32>) {
        if let Some(r) = root {
            let r = r.borrow();
            if r.left.is_none() && r.right.is_none() {
                res.push(r.val);
            }
            Solution::dfs(&r.left, res);
            Solution::dfs(&r.right, res);
        }
    }
}
