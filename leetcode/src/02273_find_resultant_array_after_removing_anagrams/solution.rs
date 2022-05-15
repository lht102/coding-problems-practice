struct Solution;

impl Solution {
    pub fn remove_anagrams(words: Vec<String>) -> Vec<String> {
        let mut res = Vec::new();
        let mut i = 0;
        while i < words.len() {
            let w = words[i].clone();
            let mut cw = words[i].chars().collect::<Vec<_>>();
            cw.sort_unstable();
            while i + 1 < words.len() {
                let mut nw = words[i + 1].chars().collect::<Vec<_>>();
                nw.sort_unstable();
                if cw != nw {
                    break;
                }
                i += 1;
            }
            res.push(w);
            i += 1;
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let words = vec![
            String::from("abba"),
            String::from("baba"),
            String::from("cd"),
            String::from("cd"),
        ];
        assert_eq!(
            Solution::remove_anagrams(words),
            vec![String::from("abba"), String::from("cd")]
        );

        let words = vec![
            String::from("a"),
            String::from("b"),
            String::from("c"),
            String::from("d"),
            String::from("e"),
        ];
        assert_eq!(
            Solution::remove_anagrams(words),
            vec![
                String::from("a"),
                String::from("b"),
                String::from("c"),
                String::from("d"),
                String::from("e"),
            ]
        );
    }
}
