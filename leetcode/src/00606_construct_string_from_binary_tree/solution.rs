use std::cell::RefCell;
use std::fmt::Write;
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
    pub fn tree2str(root: Option<Rc<RefCell<TreeNode>>>) -> String {
        let mut s = String::new();
        Solution::dfs(&root, &mut s);
        s
    }

    fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, s: &mut String) {
        if let Some(r) = root {
            let r = r.borrow();
            let _ = write!(s, "{}", r.val);
            if r.left.is_some() || r.right.is_some() {
                let _ = write!(s, "(");
                Solution::dfs(&r.left, s);
                let _ = write!(s, ")");
            }
            if r.right.is_some() {
                let _ = write!(s, "(");
                Solution::dfs(&r.right, s);
                let _ = write!(s, ")");
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    left: None,
                    right: None,
                }))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: None,
                right: None,
            }))),
        })));
        assert_eq!(Solution::tree2str(root), String::from("1(2()(4))(3)"));
    }
}
