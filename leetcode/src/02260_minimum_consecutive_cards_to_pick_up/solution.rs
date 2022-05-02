use std::collections::hash_map::Entry;
use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn minimum_card_pickup(cards: Vec<i32>) -> i32 {
        let mut map = HashMap::<i32, usize>::new();
        let mut res = usize::MAX;
        for (i, &c) in cards.iter().enumerate() {
            if let Entry::Occupied(e) = map.entry(c) {
                res = res.min(i - e.get() + 1);
            }
            map.insert(c, i);
        }
        if res == usize::MAX {
            -1
        } else {
            res as i32
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let cards = vec![3, 4, 2, 3, 4, 7];
        assert_eq!(Solution::minimum_card_pickup(cards), 4);

        let cards = vec![1, 0, 5, 3];
        assert_eq!(Solution::minimum_card_pickup(cards), -1);
    }
}
