struct Solution;

impl Solution {
    pub fn min_partitions(n: String) -> i32 {
        n.chars().map(|ch| ch as u8 - b'0').max().unwrap() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let n = String::from("32");
        assert_eq!(Solution::min_partitions(n), 3);

        let n = String::from("82734");
        assert_eq!(Solution::min_partitions(n), 8);

        let n = String::from("27346209830709182346");
        assert_eq!(Solution::min_partitions(n), 9);
    }
}
