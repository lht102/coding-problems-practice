struct Solution;

impl Solution {
    pub fn distribute_cookies(cookies: Vec<i32>, k: i32) -> i32 {
        let mut res = i32::MAX;
        let mut children = vec![0; k as usize];
        Solution::backtrack(&cookies, 0, &mut children, &mut res);
        res
    }

    fn backtrack(cookies: &[i32], i: usize, children: &mut Vec<i32>, res: &mut i32) {
        if i == cookies.len() {
            *res = (*res).min(*children.iter().max().unwrap());
            return;
        }
        if *res <= *children.iter().max().unwrap() {
            return;
        }
        for j in 0..children.len() {
            children[j] += cookies[i];
            Solution::backtrack(cookies, i + 1, children, res);
            children[j] -= cookies[i];
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let cookies = vec![8, 15, 10, 20, 8];
        let k = 2;
        assert_eq!(Solution::distribute_cookies(cookies, k), 31);

        let cookies = vec![6, 1, 3, 2, 2, 4, 1, 2];
        let k = 3;
        assert_eq!(Solution::distribute_cookies(cookies, k), 7);
    }
}
