use std::cmp::Ordering;

struct Solution;

impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut lo = 0;
        let mut hi = numbers.len() - 1;
        while lo < hi {
            let cur_sum = numbers[lo] + numbers[hi];
            match cur_sum.cmp(&target) {
                Ordering::Greater => hi -= 1,
                Ordering::Less => lo += 1,
                _ => return vec![lo as i32 + 1, hi as i32 + 1],
            }
        }
        unreachable!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let numbers = vec![2, 7, 11, 15];
        let target = 9;
        assert_eq!(Solution::two_sum(numbers, target), vec![1, 2]);

        let numbers = vec![2, 3, 4];
        let target = 6;
        assert_eq!(Solution::two_sum(numbers, target), vec![1, 3]);

        let numbers = vec![-1, 0];
        let target = -1;
        assert_eq!(Solution::two_sum(numbers, target), vec![1, 2]);
    }
}
