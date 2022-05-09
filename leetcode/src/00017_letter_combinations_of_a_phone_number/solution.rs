use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        if digits.is_empty() {
            return Vec::new();
        }
        let mut letters: HashMap<char, Vec<char>> = HashMap::new();
        letters.insert('2', vec!['a', 'b', 'c']);
        letters.insert('3', vec!['d', 'e', 'f']);
        letters.insert('4', vec!['g', 'h', 'i']);
        letters.insert('5', vec!['j', 'k', 'l']);
        letters.insert('6', vec!['m', 'n', 'o']);
        letters.insert('7', vec!['p', 'q', 'r', 's']);
        letters.insert('8', vec!['t', 'u', 'v']);
        letters.insert('9', vec!['w', 'x', 'y', 'z']);
        let mut cur = Vec::new();
        let mut res = Vec::new();
        let digits: Vec<char> = digits.chars().collect();
        Self::backtracking(&digits, &letters, 0, &mut cur, &mut res);
        res
    }

    fn backtracking(
        digits: &[char],
        letters: &HashMap<char, Vec<char>>,
        idx: usize,
        cur: &mut Vec<char>,
        res: &mut Vec<String>,
    ) {
        if idx == digits.len() {
            res.push(cur.iter().collect());
            return;
        }
        for &ch in letters.get(&digits[idx]).unwrap_or(&Vec::new()) {
            cur.push(ch);
            Self::backtracking(digits, letters, idx + 1, cur, res);
            cur.pop();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let digits = String::from("23");
        assert_eq!(
            Solution::letter_combinations(digits),
            vec![
                String::from("ad"),
                String::from("ae"),
                String::from("af"),
                String::from("bd"),
                String::from("be"),
                String::from("bf"),
                String::from("cd"),
                String::from("ce"),
                String::from("cf"),
            ],
        );

        let digits = String::from("");
        assert_eq!(Solution::letter_combinations(digits), Vec::<String>::new());

        let digits = String::from("2");
        assert_eq!(
            Solution::letter_combinations(digits),
            vec![String::from("a"), String::from("b"), String::from("c")]
        );
    }
}
