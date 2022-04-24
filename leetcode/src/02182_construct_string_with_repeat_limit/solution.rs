use std::collections::BinaryHeap;
use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn repeat_limited_string(s: String, repeat_limit: i32) -> String {
        let repeat_limit = repeat_limit as usize;
        let freqs: Vec<(char, usize)> = s
            .chars()
            .fold(HashMap::<char, usize>::new(), |mut m, ch| {
                *m.entry(ch).or_default() += 1;
                m
            })
            .into_iter()
            .collect();
        let mut max_heap: BinaryHeap<_> = BinaryHeap::from(freqs);
        let mut res: Vec<char> = Vec::new();
        while let Some((ch, cnt)) = max_heap.pop() {
            let max_use = cnt.min(repeat_limit);
            let rem = cnt - max_use;
            res.extend(vec![ch; max_use].into_iter());
            if rem > 0 {
                if let Some((next_ch, next_cnt)) = max_heap.pop() {
                    let next_rem = next_cnt - 1;
                    res.extend(vec![next_ch; 1].into_iter());
                    if next_rem > 0 {
                        max_heap.push((next_ch, next_rem));
                    }
                } else {
                    break;
                }
                max_heap.push((ch, rem));
            }
        }
        res.iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let s = String::from("cczazcc");
        let repeat_limit = 3;
        assert_eq!(
            Solution::repeat_limited_string(s, repeat_limit),
            String::from("zzcccac")
        );

        let s = String::from("aababab");
        let repeat_limit = 2;
        assert_eq!(
            Solution::repeat_limited_string(s, repeat_limit),
            String::from("bbabaa")
        );
    }
}
