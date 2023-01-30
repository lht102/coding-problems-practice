use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn digit_count(num: String) -> bool {
        let nums = num
            .chars()
            .map(|ch| (ch as u8 - b'0') as usize)
            .collect::<Vec<_>>();
        let freq = nums
            .iter()
            .fold(HashMap::<usize, usize>::new(), |mut map, &num| {
                *map.entry(num).or_default() += 1;
                map
            });
        for (i, &num) in nums.iter().enumerate() {
            let cnt = *freq.get(&i).unwrap_or(&0);
            if cnt != num {
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
        let num = String::from("1210");
        assert!(Solution::digit_count(num));

        let num = String::from("030");
        assert!(!Solution::digit_count(num));
    }
}
