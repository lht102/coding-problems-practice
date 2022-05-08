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
    pub fn average_of_subtree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut res = 0;
        Self::dfs(&root, &mut res);
        res
    }

    fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, res: &mut i32) -> (i32, usize) {
        if let Some(v) = root {
            let v = v.borrow();
            let (left_sum, left_cnt) = Self::dfs(&v.left, res);
            let (right_sum, right_cnt) = Self::dfs(&v.right, res);
            let sum = v.val + left_sum + right_sum;
            let cnt = 1 + left_cnt + right_cnt;
            if v.val == sum / cnt as i32 {
                *res += 1;
            }
            (sum, cnt)
        } else {
            (0, 0)
        }
    }
}
