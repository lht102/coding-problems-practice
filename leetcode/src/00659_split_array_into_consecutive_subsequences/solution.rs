use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn is_possible(nums: Vec<i32>) -> bool {
        let mut freq = nums
            .iter()
            .fold(HashMap::<i32, usize>::new(), |mut map, &num| {
                *map.entry(num).or_default() += 1;
                map
            });
        let mut end_freq: HashMap<i32, usize> = HashMap::new();
        for num in &nums {
            if let Some(cnt) = freq.get_mut(num) {
                if *cnt == 0 {
                    continue;
                }
                *cnt -= 1;
                if let Some(end_cnt) = end_freq.get_mut(&(num - 1)) {
                    if *end_cnt > 0 {
                        *end_cnt -= 1;
                        *end_freq.entry(*num).or_default() += 1;
                        continue;
                    }
                }
                if *freq.get(&(num + 1)).unwrap_or(&0) > 0
                    && *freq.get(&(num + 2)).unwrap_or(&0) > 0
                {
                    *freq.entry(num + 1).or_default() -= 1;
                    *freq.entry(num + 2).or_default() -= 1;
                    *end_freq.entry(num + 2).or_default() += 1;
                    continue;
                }
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let nums = vec![1, 2, 3, 3, 4, 5];
        assert!(Solution::is_possible(nums));

        let nums = vec![1, 2, 3, 3, 4, 4, 5, 5];
        assert!(Solution::is_possible(nums));

        let nums = vec![1, 2, 3, 4, 4, 5];
        assert!(!Solution::is_possible(nums));

        let nums = vec![1, 2, 3, 4, 5, 5, 6, 7];
        assert!(Solution::is_possible(nums));

        let nums = vec![3, 4, 4, 5, 6, 7, 8, 9, 10, 11];
        assert!(!Solution::is_possible(nums));
    }
}
