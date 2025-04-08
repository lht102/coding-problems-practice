use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn find_lonely(nums: Vec<i32>) -> Vec<i32> {
        let freq = nums.iter().fold(HashMap::<i32, usize>::new(), |mut m, &n| {
            *m.entry(n).or_default() += 1;
            m
        });
        nums.iter()
            .cloned()
            .filter(|n| {
                !freq.contains_key(&(n - 1))
                    && !freq.contains_key(&(n + 1))
                    && *freq.get(n).unwrap() == 1
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let nums = vec![10, 6, 5, 8];
        assert_eq!(Solution::find_lonely(nums), vec![10, 8]);

        let nums = vec![1, 3, 5, 3];
        assert_eq!(Solution::find_lonely(nums), vec![1, 5]);
    }
}
