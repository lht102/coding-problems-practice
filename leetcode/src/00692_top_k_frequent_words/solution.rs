use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::iter::FromIterator;

struct Solution;

impl Solution {
    pub fn top_k_frequent(words: Vec<String>, k: i32) -> Vec<String> {
        let mut pq = BinaryHeap::from_iter(
            words
                .iter()
                .fold(HashMap::<Reverse<&str>, usize>::new(), |mut map, s| {
                    *map.entry(Reverse(s)).or_default() += 1;
                    map
                })
                .into_iter()
                .map(|(k, v)| (v, k)),
        );
        (0..k).map(|_| pq.pop().unwrap().1.0.to_owned()).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let words = vec![
            String::from("i"),
            String::from("love"),
            String::from("leetcode"),
            String::from("i"),
            String::from("love"),
            String::from("coding"),
        ];
        let k = 2;
        assert_eq!(
            Solution::top_k_frequent(words, k),
            vec![String::from("i"), String::from("love")]
        );

        let words = vec![
            String::from("the"),
            String::from("day"),
            String::from("is"),
            String::from("sunny"),
            String::from("the"),
            String::from("the"),
            String::from("the"),
            String::from("sunny"),
            String::from("is"),
            String::from("is"),
        ];
        let k = 4;
        assert_eq!(
            Solution::top_k_frequent(words, k),
            vec![
                String::from("the"),
                String::from("is"),
                String::from("sunny"),
                String::from("day"),
            ]
        );
    }
}
