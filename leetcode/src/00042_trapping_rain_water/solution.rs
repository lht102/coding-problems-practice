struct Solution;

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let mut res = 0;
        let mut st = Vec::new();
        for (right, &current_h) in height.iter().enumerate() {
            while !st.is_empty() && current_h > height[*st.last().unwrap()] {
                let lower_bound_h = height[st.pop().unwrap()];
                if st.is_empty() {
                    break;
                }
                let left = *st.last().unwrap();
                let width = (right - left - 1) as i32;
                let upper_bound_h = current_h.min(height[left]);
                res += width * (upper_bound_h - lower_bound_h);
            }
            st.push(right);
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let height = vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1];
        assert_eq!(Solution::trap(height), 6);

        let height = vec![4, 2, 0, 3, 2, 5];
        assert_eq!(Solution::trap(height), 9);

        let height = vec![2, 1, 0, 0, 0, 0, 0, 1, 2];
        assert_eq!(Solution::trap(height), 12);
    }
}
