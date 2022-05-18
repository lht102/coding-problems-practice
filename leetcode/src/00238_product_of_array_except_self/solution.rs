struct Solution;

impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let psum = std::iter::once(1)
            .chain(nums.iter().scan(1, |acc, num| {
                *acc *= num;
                Some(*acc)
            }))
            .collect::<Vec<_>>();
        let ssum = std::iter::once(1)
            .chain(nums.iter().rev().scan(1, |acc, num| {
                *acc *= num;
                Some(*acc)
            }))
            .collect::<Vec<_>>();
        let mut res = vec![0; nums.len()];
        for (i, num) in res.iter_mut().enumerate() {
            *num = psum[i] * ssum[nums.len() - i - 1];
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let nums = vec![1, 2, 3, 4];
        assert_eq!(Solution::product_except_self(nums), vec![24, 12, 8, 6]);

        let nums = vec![-1, 1, 0, -3, 3];
        assert_eq!(Solution::product_except_self(nums), vec![0, 0, 9, 0, 0]);
    }
}
