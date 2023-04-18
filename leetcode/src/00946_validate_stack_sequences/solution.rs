struct Solution;

impl Solution {
    pub fn validate_stack_sequences(pushed: Vec<i32>, popped: Vec<i32>) -> bool {
        let mut st = Vec::new();
        let mut idx = 0;
        for x in pushed {
            st.push(x);
            while !st.is_empty() && idx < popped.len() && *st.last().unwrap() == popped[idx] {
                st.pop();
                idx += 1;
            }
        }
        idx == popped.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let pushed = vec![1, 2, 3, 4, 5];
        let popped = vec![4, 5, 3, 2, 1];
        assert!(Solution::validate_stack_sequences(pushed, popped));

        let pushed = vec![1, 2, 3, 4, 5];
        let popped = vec![4, 3, 5, 1, 2];
        assert!(!Solution::validate_stack_sequences(pushed, popped));
    }
}
