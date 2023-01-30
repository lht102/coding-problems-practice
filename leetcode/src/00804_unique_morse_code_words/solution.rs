use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn unique_morse_representations(words: Vec<String>) -> i32 {
        let morse = [
            ".-", "-...", "-.-.", "-..", ".", "..-.", "--.", "....", "..", ".---", "-.-", ".-..",
            "--", "-.", "---", ".--.", "--.-", ".-.", "...", "-", "..-", "...-", ".--", "-..-",
            "-.--", "--..",
        ];
        words
            .into_iter()
            .fold(HashSet::<String>::new(), |mut word_set, w| {
                let s = w
                    .chars()
                    .flat_map(|ch| morse[(ch as u8 - b'a') as usize].chars())
                    .collect::<String>();
                word_set.insert(s);
                word_set
            })
            .len() as _
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let words = vec![
            String::from("gin"),
            String::from("zen"),
            String::from("gig"),
            String::from("msg"),
        ];
        assert_eq!(Solution::unique_morse_representations(words), 2);

        let words = vec![String::from("a")];
        assert_eq!(Solution::unique_morse_representations(words), 1);
    }
}
