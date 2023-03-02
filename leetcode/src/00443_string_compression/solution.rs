struct Solution;

impl Solution {
    pub fn compress(chars: &mut Vec<char>) -> i32 {
        let mut res = 0;
        let n = chars.len();
        let mut i = 0;
        while i < n {
            let mut group_len = 1;
            while i + group_len < n && chars[i + group_len] == chars[i] {
                group_len += 1;
            }
            chars[res] = chars[i];
            res += 1;
            if group_len > 1 {
                for ch in group_len.to_string().chars() {
                    chars[res] = ch;
                    res += 1;
                }
            }
            i += group_len;
        }
        res as _
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut chars = vec!['a', 'a', 'b', 'b', 'c', 'c', 'c'];
        assert_eq!(Solution::compress(&mut chars), 6);

        let mut chars = vec!['a'];
        assert_eq!(Solution::compress(&mut chars), 1);

        let mut chars = vec![
            'a', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b',
        ];
        assert_eq!(Solution::compress(&mut chars), 4);
    }
}
