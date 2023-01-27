struct Solution;

impl Solution {
    pub fn detect_capital_use(word: String) -> bool {
        let chs = word.chars().collect::<Vec<_>>();
        let first_cap = chs.first().unwrap().is_uppercase();
        let cap_cnt = chs.iter().filter(|ch| ch.is_uppercase()).count();
        first_cap && cap_cnt == 1 || cap_cnt == chs.len() || cap_cnt == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let word = String::from("USA");
        assert!(Solution::detect_capital_use(word));

        let word = String::from("FlaG");
        assert!(!Solution::detect_capital_use(word));
    }
}
