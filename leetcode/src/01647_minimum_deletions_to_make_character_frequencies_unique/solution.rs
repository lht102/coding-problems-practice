use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::iter::FromIterator;

struct Solution;

impl Solution {
    pub fn min_deletions(s: String) -> i32 {
        let mut pq = BinaryHeap::from_iter(
            s.chars()
                .fold(HashMap::<char, usize>::new(), |mut map, ch| {
                    *map.entry(ch).or_default() += 1;
                    map
                })
                .into_values(),
        );
        let mut res = 0;
        while pq.len() > 1 {
            let top = pq.pop().unwrap();
            if top == *pq.peek().unwrap() {
                if top > 1 {
                    pq.push(top - 1);
                }
                res += 1;
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
        let s = String::from("aab");
        assert_eq!(Solution::min_deletions(s), 0);

        let s = String::from("aaabbbcc");
        assert_eq!(Solution::min_deletions(s), 2);

        let s = String::from("ceabaacb");
        assert_eq!(Solution::min_deletions(s), 2);
    }
}
