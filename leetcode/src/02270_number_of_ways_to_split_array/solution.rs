struct Solution;

impl Solution {
    pub fn ways_to_split_array(nums: Vec<i32>) -> i32 {
        let psum = std::iter::once(0)
            .chain(nums.iter().scan(0, |sum, num| {
                *sum += num;
                Some(*sum)
            }))
            .collect::<Vec<_>>();
        let mut res = 0;
        for i in 1..nums.len() {
            let pre = psum[i];
            let suf = psum[nums.len()] - psum[i];
            if pre >= suf {
                res += 1;
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let nums = vec![10, 4, -8, 7];
        assert_eq!(Solution::ways_to_split_array(nums), 2);

        let nums = vec![2, 3, 1, 0];
        assert_eq!(Solution::ways_to_split_array(nums), 2);
    }
}
