use std::cell::RefCell;
use std::collections::BTreeMap;
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
    pub fn vertical_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut map = BTreeMap::new();
        Solution::dfs(&root, 0, 0, &mut map);
        map.into_values()
            .map(|mut pairs| {
                pairs.sort_unstable();
                pairs.into_iter().map(|(_, val)| val).collect()
            })
            .collect()
    }

    fn dfs(
        root: &Option<Rc<RefCell<TreeNode>>>,
        level_v: usize,
        level_h: i32,
        map: &mut BTreeMap<i32, Vec<(usize, i32)>>,
    ) {
        if let Some(r) = root {
            let r = r.borrow();
            map.entry(level_h).or_default().push((level_v, r.val));
            Solution::dfs(&r.left, level_v + 1, level_h - 1, map);
            Solution::dfs(&r.right, level_v + 1, level_h + 1, map);
        }
    }
}
