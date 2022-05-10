struct Solution;

impl Solution {
    pub fn combination_sum3(k: i32, n: i32) -> Vec<Vec<i32>> {
        let mut cur = Vec::new();
        let mut res = Vec::new();
        Self::backtracking(k, n, 1, &mut cur, &mut res);
        res
    }

    fn backtracking(k: i32, n: i32, num: i32, cur: &mut Vec<i32>, res: &mut Vec<Vec<i32>>) {
        if n < 0 {
            return;
        }
        if k == 0 && n == 0 {
            res.push(cur.clone());
            return;
        }
        for i in num..=9 {
            cur.push(i);
            Self::backtracking(k - 1, n - i, i + 1, cur, res);
            cur.pop();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let k = 3;
        let n = 7;
        assert_eq!(Solution::combination_sum3(k, n), vec![vec![1, 2, 4]]);

        let k = 3;
        let n = 9;
        assert_eq!(
            Solution::combination_sum3(k, n),
            vec![vec![1, 2, 6], vec![1, 3, 5], vec![2, 3, 4]]
        );

        let k = 4;
        let n = 1;
        assert_eq!(Solution::combination_sum3(k, n), Vec::<Vec<i32>>::new());
    }
}
