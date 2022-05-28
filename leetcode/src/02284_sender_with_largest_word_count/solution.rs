use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn largest_word_count(messages: Vec<String>, senders: Vec<String>) -> String {
        let mut list = messages
            .iter()
            .zip(senders.iter())
            .fold(HashMap::<String, usize>::new(), |mut freq, (m, s)| {
                *freq.entry(s.clone()).or_default() +=
                    m.chars().filter(|&ch| ch == ' ').count() + 1;
                freq
            })
            .into_iter()
            .map(|(name, cnt)| (cnt, name))
            .collect::<Vec<_>>();
        list.sort_unstable();
        list.last().unwrap().1.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let messages = vec![
            String::from("Hello userTwoo"),
            String::from("Hi userThree"),
            String::from("Wonderful day Alice"),
            String::from("Nice day userThree"),
        ];
        let senders = vec![
            String::from("Alice"),
            String::from("userTwo"),
            String::from("userThree"),
            String::from("Alice"),
        ];
        assert_eq!(
            Solution::largest_word_count(messages, senders),
            String::from("Alice")
        );

        let messages = vec![
            String::from("How is leetcode for everyone"),
            String::from("Leetcode is useful for practice"),
        ];
        let senders = vec![String::from("Bob"), String::from("Charlie")];
        assert_eq!(
            Solution::largest_word_count(messages, senders),
            String::from("Charlie")
        );
    }
}
