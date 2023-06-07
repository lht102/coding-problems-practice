use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn can_make_arithmetic_progression(arr: Vec<i32>) -> bool {
        let min_v = *arr.iter().min().unwrap();
        let max_v = *arr.iter().max().unwrap();
        let n = arr.len() as i32;
        if max_v - min_v == 0 {
            return true;
        }
        if (max_v - min_v) % (n - 1) != 0 {
            return false;
        }
        let diff = (max_v - min_v) / (n - 1);
        let mut visited = HashSet::new();
        for &num in &arr {
            if (num - min_v) % diff != 0 {
                return false;
            }
            visited.insert(num);
        }
        visited.len() == arr.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let arr = vec![3, 5, 1];
        assert!(Solution::can_make_arithmetic_progression(arr));

        let arr = vec![1, 2, 4];
        assert!(!Solution::can_make_arithmetic_progression(arr));

        let arr = vec![1, 2, 3, 2, 5];
        assert!(!Solution::can_make_arithmetic_progression(arr));
    }
}
