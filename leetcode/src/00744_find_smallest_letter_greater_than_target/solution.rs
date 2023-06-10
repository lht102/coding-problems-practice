struct Solution;

impl Solution {
    pub fn next_greatest_letter(letters: Vec<char>, target: char) -> char {
        let idx = letters.partition_point(|&x| x <= target);
        if idx == letters.len() {
            letters[0]
        } else {
            letters[idx]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let letters = vec!['c', 'f', 'j'];
        let target = 'a';
        assert_eq!(Solution::next_greatest_letter(letters, target), 'c');

        let letters = vec!['c', 'f', 'j'];
        let target = 'c';
        assert_eq!(Solution::next_greatest_letter(letters, target), 'f');

        let letters = vec!['x', 'x', 'y', 'y'];
        let target = 'z';
        assert_eq!(Solution::next_greatest_letter(letters, target), 'x');
    }
}
