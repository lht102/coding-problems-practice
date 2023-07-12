use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
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
    pub fn distance_k(
        root: Option<Rc<RefCell<TreeNode>>>,
        target: Option<Rc<RefCell<TreeNode>>>,
        k: i32,
    ) -> Vec<i32> {
        let mut graph = HashMap::new();
        Self::build_graph(&root, None, &mut graph);
        let mut res = Vec::new();
        Self::dfs(
            &graph,
            k,
            target.unwrap().borrow().val,
            &mut HashSet::new(),
            &mut res,
        );
        res
    }

    fn build_graph(
        root: &Option<Rc<RefCell<TreeNode>>>,
        parent: Option<i32>,
        graph: &mut HashMap<i32, Vec<i32>>,
    ) {
        if let Some(r) = root {
            let r = r.borrow();
            if let Some(p) = parent {
                graph.entry(r.val).or_default().push(p);
                graph.entry(p).or_default().push(r.val);
            }
            Self::build_graph(&r.left, Some(r.val), graph);
            Self::build_graph(&r.right, Some(r.val), graph);
        }
    }

    fn dfs(
        graph: &HashMap<i32, Vec<i32>>,
        k: i32,
        source: i32,
        visited: &mut HashSet<i32>,
        res: &mut Vec<i32>,
    ) {
        if visited.contains(&source) {
            return;
        }
        visited.insert(source);
        if k == 0 {
            res.push(source);
        }
        for &destination in graph.get(&source).unwrap_or(&vec![]) {
            Self::dfs(graph, k - 1, destination, visited, res);
        }
    }
}
