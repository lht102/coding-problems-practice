struct Solution;

impl Solution {
    pub fn find132pattern(nums: Vec<i32>) -> bool {
        let mut num_k = i32::MIN;
        let mut st = Vec::new();
        for &num in nums.iter().rev() {
            if num < num_k {
                return true;
            }
            while let Some(&last) = st.last() {
                if num <= last {
                    break;
                }
                num_k = st.pop().unwrap();
            }
            st.push(num);
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let nums = vec![1, 2, 3, 4];
        assert!(!Solution::find132pattern(nums));

        let nums = vec![3, 1, 4, 2];
        assert!(Solution::find132pattern(nums));

        let nums = vec![-1, 3, 2, 0];
        assert!(Solution::find132pattern(nums));

        let nums = vec![3, 5, 0, 3, 4];
        assert!(Solution::find132pattern(nums));
    }
}
