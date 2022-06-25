use std::collections::BinaryHeap;

struct Solution;

impl Solution {
    pub fn is_possible(target: Vec<i32>) -> bool {
        let mut total: i32 = target.iter().sum();
        let mut pq = BinaryHeap::from(target);
        while let Some(top) = pq.pop() {
            if top == 1 {
                return true;
            }
            if top == total {
                return false;
            }
            let rem = (top - 1) % (total - top) + 1;
            if rem == top {
                return false;
            }
            total = total + rem - top;
            pq.push(rem);
        }
        unreachable!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let target = vec![9, 3, 5];
        assert!(Solution::is_possible(target));

        let target = vec![1, 1, 1, 2];
        assert!(!Solution::is_possible(target));

        let target = vec![8, 5];
        assert!(Solution::is_possible(target));
    }
}
