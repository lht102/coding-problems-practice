use std::collections::BTreeSet;
use std::collections::HashMap;
use std::iter::FromIterator;

struct Solution;

impl Solution {
    pub fn num_factored_binary_trees(arr: Vec<i32>) -> i32 {
        const M: i32 = 1_000_000_007;
        let set = BTreeSet::from_iter(arr.iter().cloned());
        let mut dp = HashMap::new();
        arr.into_iter().fold(0, |acc, num| {
            (acc + Solution::solve(&set, num, &mut dp)) % M
        })
    }

    fn solve(set: &BTreeSet<i32>, root: i32, dp: &mut HashMap<i32, i32>) -> i32 {
        const M: i32 = 1_000_000_007;
        if let Some(&val) = dp.get(&root) {
            return val;
        }
        let mut res = 1;
        for &first_child in set {
            let second_child = root / first_child;
            if root % first_child == 0 && set.contains(&second_child) {
                res = (res
                    + (Solution::solve(set, first_child, dp) as i64
                        * Solution::solve(set, second_child, dp) as i64
                        % M as i64) as i32)
                    % M;
            }
        }
        dp.insert(root, res);
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let arr = vec![2, 4];
        assert_eq!(Solution::num_factored_binary_trees(arr), 3);

        let arr = vec![2, 4, 5, 10];
        assert_eq!(Solution::num_factored_binary_trees(arr), 7);
    }
}
