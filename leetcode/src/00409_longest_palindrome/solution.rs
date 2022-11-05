use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn longest_palindrome(s: String) -> i32 {
        let num_of_odd = s
            .chars()
            .fold(HashMap::<char, usize>::new(), |mut map, ch| {
                *map.entry(ch).or_default() += 1;
                map
            })
            .into_values()
            .filter(|cnt| cnt % 2 == 1)
            .count();
        (s.len() - num_of_odd + usize::from(num_of_odd > 0)) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let s = String::from("abccccdd");
        assert_eq!(Solution::longest_palindrome(s), 7);

        let s = String::from("a");
        assert_eq!(Solution::longest_palindrome(s), 1);

        let s = String::from("bb");
        assert_eq!(Solution::longest_palindrome(s), 2);
    }
}
