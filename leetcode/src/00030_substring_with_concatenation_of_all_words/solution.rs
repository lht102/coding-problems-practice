use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
        let word_len = words[0].len();
        let total_len = word_len * words.len();
        let freq = words
            .iter()
            .fold(HashMap::<&str, usize>::new(), |mut map, s| {
                *map.entry(s).or_default() += 1;
                map
            });
        s.chars()
            .collect::<Vec<_>>()
            .windows(total_len)
            .into_iter()
            .enumerate()
            .filter_map(|(i, w)| Solution::is_valid(freq.clone(), w, word_len).then_some(i as i32))
            .collect::<Vec<_>>()
    }

    fn is_valid(mut freq: HashMap<&str, usize>, w: &[char], word_len: usize) -> bool {
        for c in w.chunks(word_len) {
            let s = c.iter().collect::<String>();
            match freq.get_mut(s.as_str()) {
                Some(cnt) if *cnt == 0 => return false,
                None => return false,
                Some(cnt) => *cnt -= 1,
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
        let s = String::from("barfoothefoobarman");
        let words = vec![String::from("foo"), String::from("bar")];
        assert_eq!(Solution::find_substring(s, words), vec![0, 9]);

        let s = String::from("wordgoodgoodgoodbestword");
        let words = vec![
            String::from("word"),
            String::from("good"),
            String::from("best"),
            String::from("word"),
        ];
        assert_eq!(Solution::find_substring(s, words), vec![]);

        let s = String::from("barfoofoobarthefoobarman");
        let words = vec![
            String::from("bar"),
            String::from("foo"),
            String::from("the"),
        ];
        assert_eq!(Solution::find_substring(s, words), vec![6, 9, 12]);
    }
}
