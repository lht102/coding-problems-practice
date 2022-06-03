struct Solution;

impl Solution {
    pub fn total_steps(nums: Vec<i32>) -> i32 {
        let mut res = 0;
        let mut st = vec![(nums[0], 0)];
        for &num in nums.iter().skip(1) {
            let mut cnt = 0;
            while let Some(&(cur_num, cur_cnt)) = st.last() {
                if cur_num > num {
                    break;
                }
                cnt = cnt.max(cur_cnt);
                st.pop();
            }
            if !st.is_empty() {
                cnt += 1;
            }
            res = res.max(cnt);
            st.push((num, cnt));
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let nums = vec![5, 3, 4, 4, 7, 3, 6, 11, 8, 5, 11];
        assert_eq!(Solution::total_steps(nums), 3);

        let nums = vec![4, 5, 7, 7, 13];
        assert_eq!(Solution::total_steps(nums), 0);
    }
}
