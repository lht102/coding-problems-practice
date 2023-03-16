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
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        let num_to_idx = inorder
            .iter()
            .enumerate()
            .map(|(i, &v)| (v, i))
            .collect::<HashMap<i32, usize>>();
        Self::dfs(&preorder, &num_to_idx, &mut 0, 0, inorder.len() as i32 - 1)
    }

    fn dfs(
        preorder: &[i32],
        in_num_to_idx: &HashMap<i32, usize>,
        pre_idx: &mut usize,
        in_left: i32,
        in_right: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if in_left > in_right {
            return None;
        }
        let val = preorder[*pre_idx];
        let mid = *in_num_to_idx.get(&val).unwrap() as i32;
        *pre_idx += 1;
        let root = Rc::new(RefCell::new(TreeNode::new(val)));
        root.borrow_mut().left = Self::dfs(preorder, in_num_to_idx, pre_idx, in_left, mid - 1);
        root.borrow_mut().right = Self::dfs(preorder, in_num_to_idx, pre_idx, mid + 1, in_right);
        Some(root)
    }
}
