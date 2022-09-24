struct Solution;

impl Solution {
    pub fn sum_even_after_queries(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut nums = nums;
        queries
            .into_iter()
            .scan(
                nums.iter().filter(|&x| x % 2 == 0).sum::<i32>(),
                |even_sum, q| {
                    let (val, i) = (q[0], q[1] as usize);
                    if nums[i] % 2 == 0 {
                        *even_sum -= nums[i];
                    }
                    nums[i] += val;
                    if nums[i] % 2 == 0 {
                        *even_sum += nums[i];
                    }
                    Some(*even_sum)
                },
            )
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let nums = vec![1, 2, 3, 4];
        let queries = vec![vec![1, 0], vec![-3, 1], vec![-4, 0], vec![2, 3]];
        assert_eq!(
            Solution::sum_even_after_queries(nums, queries),
            vec![8, 6, 2, 4]
        );

        let nums = vec![1];
        let queries = vec![vec![4, 0]];
        assert_eq!(Solution::sum_even_after_queries(nums, queries), vec![0]);
    }
}
