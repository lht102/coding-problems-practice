use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn count_vowel_permutation(n: i32) -> i32 {
        let rules = HashMap::from([
            (' ', vec!['a', 'e', 'i', 'o', 'u']),
            ('a', vec!['e']),
            ('e', vec!['a', 'i']),
            ('i', vec!['a', 'e', 'o', 'u']),
            ('o', vec!['i', 'u']),
            ('u', vec!['a']),
        ]);
        let mut dp = vec![HashMap::<char, i32>::new(); n as usize + 1];
        Solution::solve(n, ' ', &rules, &mut dp)
    }

    fn solve(
        n: i32,
        prev: char,
        rules: &HashMap<char, Vec<char>>,
        dp: &mut Vec<HashMap<char, i32>>,
    ) -> i32 {
        if n == 0 {
            return 1;
        }
        if let Some(&val) = dp[n as usize].get(&prev) {
            return val;
        }
        let mut res = 0;
        for &cur in rules.get(&prev).unwrap_or(&vec![]) {
            res = (res + Solution::solve(n - 1, cur, rules, dp) % 1_000_000_007) % 1_000_000_007;
        }
        dp[n as usize].insert(prev, res);
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let n = 1;
        assert_eq!(Solution::count_vowel_permutation(n), 5);

        let n = 2;
        assert_eq!(Solution::count_vowel_permutation(n), 10);

        let n = 5;
        assert_eq!(Solution::count_vowel_permutation(n), 68);
    }
}
