use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn num_matching_subseq(s: String, words: Vec<String>) -> i32 {
        let char_indices =
            s.char_indices()
                .fold(HashMap::<char, Vec<usize>>::new(), |mut map, (i, ch)| {
                    map.entry(ch).or_default().push(i);
                    map
                });
        words
            .iter()
            .map(|w| i32::from(Solution::is_subsequence(w, &char_indices)))
            .sum()
    }

    fn is_subsequence(w: &str, char_indices: &HashMap<char, Vec<usize>>) -> bool {
        let mut cur_idx = 0;
        for ch in w.chars() {
            if let Some(indices) = char_indices.get(&ch) {
                let j = indices.partition_point(|&x| x < cur_idx);
                if j >= indices.len() {
                    return false;
                }
                cur_idx = indices[j] + 1;
            } else {
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
        let s = String::from("abcde");
        let words = vec![
            String::from("a"),
            String::from("bb"),
            String::from("acd"),
            String::from("ace"),
        ];
        assert_eq!(Solution::num_matching_subseq(s, words), 3);

        let s = String::from("dsahjpjauf");
        let words = vec![
            String::from("ahjpjau"),
            String::from("ja"),
            String::from("ahbwzgqnuk"),
            String::from("tnmlanowax"),
        ];
        assert_eq!(Solution::num_matching_subseq(s, words), 2);
    }
}
