use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::iter::FromIterator;

struct Solution;

impl Solution {
    pub fn find_ladders(
        begin_word: String,
        end_word: String,
        word_list: Vec<String>,
    ) -> Vec<Vec<String>> {
        let begin_word_bytes = begin_word.bytes().collect::<Vec<_>>();
        let mut end_word_bytes = end_word.bytes().collect::<Vec<_>>();
        let word_set: HashSet<_> =
            HashSet::from_iter(word_list.into_iter().map(|w| w.bytes().collect::<Vec<_>>()));
        let mut q = VecDeque::from([begin_word_bytes.clone()]);
        let mut word_distance = HashMap::from([(begin_word_bytes.clone(), 0)]);
        while let Some(front) = q.pop_front() {
            let distance = *word_distance.entry(front.clone()).or_default();
            let mut next = front;
            for i in 0..next.len() {
                let tmp = next[i];
                for ch in b'a'..=b'z' {
                    next[i] = ch;
                    if word_set.contains(&next) && !word_distance.contains_key(&next) {
                        word_distance.insert(next.clone(), distance + 1);
                        q.push_back(next.clone());
                    }
                }
                next[i] = tmp;
            }
        }
        let mut res = Vec::new();
        if word_distance.contains_key(&end_word_bytes) {
            let mut cur_path = Vec::new();
            Solution::backtrack(
                &word_distance,
                &begin_word_bytes,
                &mut end_word_bytes,
                &mut cur_path,
                &mut res,
            );
        }
        res
    }

    fn backtrack(
        word_distance: &HashMap<Vec<u8>, i32>,
        begin_word_bytes: &[u8],
        end_word_bytes: &mut [u8],
        cur_path: &mut Vec<Vec<u8>>,
        res: &mut Vec<Vec<String>>,
    ) {
        if begin_word_bytes == end_word_bytes {
            let mut path = cur_path
                .iter()
                .cloned()
                .map(|arr| String::from_utf8(arr).unwrap())
                .collect::<Vec<_>>();
            path.push(String::from_utf8(begin_word_bytes.iter().cloned().collect()).unwrap());
            path.reverse();
            res.push(path);
            return;
        }
        cur_path.push(end_word_bytes.iter().cloned().collect::<Vec<_>>());
        let cur_distance = *word_distance.get(end_word_bytes).unwrap();
        for i in 0..end_word_bytes.len() {
            let tmp = end_word_bytes[i];
            for ch in b'a'..=b'z' {
                end_word_bytes[i] = ch;
                if let Some(&prev_distance) = word_distance.get(end_word_bytes) {
                    if prev_distance == cur_distance - 1 {
                        Solution::backtrack(
                            word_distance,
                            begin_word_bytes,
                            end_word_bytes,
                            cur_path,
                            res,
                        );
                    }
                }
            }
            end_word_bytes[i] = tmp;
        }
        cur_path.pop();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let begin_word = String::from("hit");
        let end_word = String::from("cog");
        let word_list = vec![
            String::from("hot"),
            String::from("dot"),
            String::from("dog"),
            String::from("lot"),
            String::from("log"),
            String::from("cog"),
        ];
        assert_eq!(
            Solution::find_ladders(begin_word, end_word, word_list),
            vec![
                vec![
                    String::from("hit"),
                    String::from("hot"),
                    String::from("dot"),
                    String::from("dog"),
                    String::from("cog")
                ],
                vec![
                    String::from("hit"),
                    String::from("hot"),
                    String::from("lot"),
                    String::from("log"),
                    String::from("cog")
                ]
            ]
        );

        let begin_word = String::from("hit");
        let end_word = String::from("cog");
        let word_list = vec![
            String::from("hot"),
            String::from("dot"),
            String::from("dog"),
            String::from("lot"),
            String::from("log"),
        ];
        assert_eq!(
            Solution::find_ladders(begin_word, end_word, word_list),
            Vec::<Vec<String>>::new()
        );
    }
}
