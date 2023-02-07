use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn total_fruit(fruits: Vec<i32>) -> i32 {
        let (mut start, mut end) = (0, 0);
        let mut freq = HashMap::<i32, usize>::new();
        while end < fruits.len() {
            *freq.entry(fruits[end]).or_default() += 1;
            if freq.len() > 2 {
                let num = fruits[start];
                if *freq.entry(num).and_modify(|e| *e -= 1).or_default() == 0 {
                    freq.remove(&num);
                }
                start += 1;
            }
            end += 1;
        }
        (end - start) as _
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let fruits = vec![1, 2, 1];
        assert_eq!(Solution::total_fruit(fruits), 3);

        let fruits = vec![0, 1, 2, 2];
        assert_eq!(Solution::total_fruit(fruits), 3);

        let fruits = vec![1, 2, 3, 2, 2];
        assert_eq!(Solution::total_fruit(fruits), 4);
    }
}
