struct Solution;

impl Solution {
    pub fn maximum_importance(n: i32, roads: Vec<Vec<i32>>) -> i64 {
        let n = n as usize;
        let mut ind = vec![0; n];
        for r in &roads {
            ind[r[0] as usize] += 1;
            ind[r[1] as usize] += 1;
        }
        ind.sort_unstable();
        let mut res = 0;
        for i in 0..n {
            res += ind[i] * (i as i64 + 1);
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let n = 5;
        let roads = vec![
            vec![0, 1],
            vec![1, 2],
            vec![2, 3],
            vec![0, 2],
            vec![1, 3],
            vec![2, 4],
        ];
        assert_eq!(Solution::maximum_importance(n, roads), 43);

        let n = 5;
        let roads = vec![vec![0, 3], vec![2, 4], vec![1, 3]];
        assert_eq!(Solution::maximum_importance(n, roads), 20);
    }
}
