struct Solution;

impl Solution {
    pub fn permute_unique(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums = nums;
        nums.sort_unstable();
        let mut vi = vec![false; nums.len()];
        let mut cur = Vec::new();
        let mut res = Vec::new();
        Self::dfs(&nums, &mut vi, &mut cur, &mut res);
        res
    }

    fn dfs(nums: &[i32], vi: &mut Vec<bool>, cur: &mut Vec<i32>, res: &mut Vec<Vec<i32>>) {
        if cur.len() == nums.len() {
            res.push(cur.clone());
            return;
        }
        for i in 0..nums.len() {
            if vi[i] || i > 0 && nums[i - 1] == nums[i] && !vi[i - 1] {
                continue;
            }
            vi[i] = true;
            cur.push(nums[i]);
            Self::dfs(nums, vi, cur, res);
            cur.pop();
            vi[i] = false;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let nums = vec![1, 1, 2];
        assert_eq!(
            Solution::permute_unique(nums),
            vec![vec![1, 1, 2], vec![1, 2, 1], vec![2, 1, 1]]
        );

        let nums = vec![1, 2, 3];
        assert_eq!(
            Solution::permute_unique(nums),
            vec![
                vec![1, 2, 3],
                vec![1, 3, 2],
                vec![2, 1, 3],
                vec![2, 3, 1],
                vec![3, 1, 2],
                vec![3, 2, 1]
            ]
        );
    }
}
