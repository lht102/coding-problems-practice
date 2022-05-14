struct Solution;

impl Solution {
    pub fn divisor_substrings(num: i32, k: i32) -> i32 {
        let chs_to_num = |sl: &[char]| {
            let mut res = 0;
            for &ch in sl {
                res = res * 10 + (ch as u8 - b'0') as i32
            }
            res
        };

        num.to_string()
            .chars()
            .collect::<Vec<_>>()
            .windows(k as usize)
            .filter(|&w| {
                let divisor = chs_to_num(w);
                divisor != 0 && num % divisor == 0
            })
            .count() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test() {
        let num = 240;
        let k = 2;
        assert_eq!(Solution::divisor_substrings(num, k), 2);

        let num = 430043;
        let k = 2;
        assert_eq!(Solution::divisor_substrings(num, k), 2);
    }
}
