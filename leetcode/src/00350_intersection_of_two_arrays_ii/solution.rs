use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut freq = nums1
            .into_iter()
            .fold(HashMap::<i32, usize>::new(), |mut map, x| {
                *map.entry(x).or_default() += 1;
                map
            });
        let mut res = Vec::new();

        for num in nums2.iter() {
            if let Some(cnt) = freq.get_mut(num) {
                if *cnt > 0 {
                    res.push(*num);
                    *cnt -= 1;
                }
            }
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let nums1 = vec![1, 2, 2, 1];
        let nums2 = vec![2, 2];
        let mut actual = Solution::intersect(nums1, nums2);
        actual.sort_unstable();
        assert_eq!(actual, vec![2, 2]);

        let nums1 = vec![4, 9, 5];
        let nums2 = vec![9, 4, 9, 8, 4];
        let mut actual = Solution::intersect(nums1, nums2);
        actual.sort_unstable();
        assert_eq!(actual, vec![4, 9]);
    }
}
