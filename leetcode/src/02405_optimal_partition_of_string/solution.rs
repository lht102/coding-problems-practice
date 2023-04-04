struct Solution;

impl Solution {
    pub fn partition_string(s: String) -> i32 {
        let mut mask = 0;
        s.bytes().fold(1, |mut res, ch| {
            let ch_mask = 1 << (ch - b'a');
            if mask & ch_mask > 0 {
                res += 1;
                mask = 0;
            }
            mask |= ch_mask;
            res
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let s = String::from("abacaba");
        assert_eq!(Solution::partition_string(s), 4);

        let s = String::from("ssssss");
        assert_eq!(Solution::partition_string(s), 6);
    }
}
