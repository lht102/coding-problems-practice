struct Solution;

impl Solution {
    pub fn lexical_order(n: i32) -> Vec<i32> {
        let mut res = Vec::new();
        for i in 1..10 {
            Self::dfs(i, n, &mut res);
        }
        res
    }

    fn dfs(cur_num: i32, n: i32, res: &mut Vec<i32>) {
        if cur_num > n {
            return;
        }
        res.push(cur_num);
        for i in 0..10 {
            let next_num = cur_num * 10 + i;
            if next_num > n {
                break;
            }
            Self::dfs(next_num, n, res);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexical_order() {
        assert_eq!(
            Solution::lexical_order(13),
            vec![1, 10, 11, 12, 13, 2, 3, 4, 5, 6, 7, 8, 9]
        );
        assert_eq!(Solution::lexical_order(2), vec![1, 2]);
    }
}
