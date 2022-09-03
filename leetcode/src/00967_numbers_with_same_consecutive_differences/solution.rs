use std::collections::VecDeque;
use std::iter::FromIterator;

struct Solution;

impl Solution {
    pub fn nums_same_consec_diff(n: i32, k: i32) -> Vec<i32> {
        if n == 1 {
            return (0..10).collect();
        }
        let mut q = VecDeque::from_iter(1..10);
        for _ in 1..n {
            for _ in 0..q.len() {
                if let Some(front) = q.pop_front() {
                    let digit = front % 10;
                    let mut path = vec![digit - k];
                    if k != 0 {
                        path.push(digit + k);
                    }
                    for next_digit in path {
                        if (0..10).contains(&next_digit) {
                            q.push_back(front * 10 + next_digit);
                        }
                    }
                }
            }
        }
        q.into_iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let n = 3;
        let k = 7;
        assert_eq!(
            Solution::nums_same_consec_diff(n, k),
            vec![181, 292, 707, 818, 929]
        );

        let n = 2;
        let k = 1;
        assert_eq!(
            Solution::nums_same_consec_diff(n, k),
            vec![10, 12, 21, 23, 32, 34, 43, 45, 54, 56, 65, 67, 76, 78, 87, 89, 98]
        );

        let n = 1;
        let k = 1;
        assert_eq!(
            Solution::nums_same_consec_diff(n, k),
            (0..10).collect::<Vec<_>>()
        );
    }
}
