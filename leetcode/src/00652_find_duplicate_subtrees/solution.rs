use std::cell::RefCell;
use std::collections::HashMap;
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
    pub fn find_duplicate_subtrees(
        root: Option<Rc<RefCell<TreeNode>>>,
    ) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        let mut res = Vec::new();
        Self::dfs(&root, &mut HashMap::new(), &mut res);
        res
    }

    fn dfs(
        root: &Option<Rc<RefCell<TreeNode>>>,
        freq: &mut HashMap<String, usize>,
        res: &mut Vec<Option<Rc<RefCell<TreeNode>>>>,
    ) -> String {
        if let Some(r) = root {
            let r = r.borrow();
            let left = Self::dfs(&r.left, freq, res);
            let right = Self::dfs(&r.right, freq, res);

            let tree_str = format!("({}){}({})", left, r.val, right);
            let cnt = freq.entry(tree_str.clone()).or_default();
            *cnt += 1;
            if *cnt == 2 {
                res.push(root.clone());
            }

            tree_str
        } else {
            String::new()
        }
    }
}
