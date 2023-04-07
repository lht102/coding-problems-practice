struct Solution;

impl Solution {
    pub fn make_sub_k_sum_equal(arr: Vec<i32>, k: i32) -> i64 {
        let k = k as usize;
        let n = arr.len();
        let mut res = 0;
        let mut visited = vec![false; n];
        for i in 0..n {
            if visited[i] {
                continue;
            }
            let mut group = Vec::new();
            let mut idx = i;
            while !visited[idx] {
                visited[idx] = true;
                group.push(arr[idx]);
                idx = (idx + k) % n;
            }
            group.sort_unstable();
            res += group
                .iter()
                .map(|x| (x - group[group.len() / 2]).abs() as i64)
                .sum::<i64>();
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let arr = vec![1, 4, 1, 3];
        let k = 2;
        assert_eq!(Solution::make_sub_k_sum_equal(arr, k), 1);

        let arr = vec![2, 5, 5, 7];
        let k = 3;
        assert_eq!(Solution::make_sub_k_sum_equal(arr, k), 5);
    }
}
