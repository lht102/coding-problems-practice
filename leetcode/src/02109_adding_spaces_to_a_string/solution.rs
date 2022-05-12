struct Solution;

impl Solution {
    pub fn add_spaces(s: String, spaces: Vec<i32>) -> String {
        let mut res: Vec<char> = std::iter::repeat('#')
            .take(s.len() + spaces.len())
            .collect();
        for (i, &idx) in spaces.iter().enumerate() {
            res[idx as usize + i] = ' ';
        }
        let s = s.chars().collect::<Vec<_>>();
        let mut j = 0;
        for ch in &mut res {
            if *ch == '#' {
                *ch = s[j];
                j += 1;
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
        let s = String::from("LeetcodeHelpsMeLearn");
        let spaces = vec![8, 13, 15];
        assert_eq!(
            Solution::add_spaces(s, spaces),
            String::from("Leetcode Helps Me Learn")
        );

        let s = String::from("icodeinpython");
        let spaces = vec![1, 5, 7, 9];
        assert_eq!(
            Solution::add_spaces(s, spaces),
            String::from("i code in py thon")
        );

        let s = String::from("spacing");
        let spaces = vec![0, 1, 2, 3, 4, 5, 6];
        assert_eq!(
            Solution::add_spaces(s, spaces),
            String::from(" s p a c i n g")
        );
    }
}
