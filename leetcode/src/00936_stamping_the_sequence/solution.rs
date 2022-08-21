struct Solution;

impl Solution {
    pub fn moves_to_stamp(stamp: String, target: String) -> Vec<i32> {
        let s = stamp.into_bytes();
        let mut t = target.into_bytes();
        let n = t.len() - s.len() + 1;
        let mut res = vec![];
        loop {
            let mut is_updated = false;
            for i in 0..n {
                if Solution::can_update(&s, &t, i) {
                    for j in 0..s.len() {
                        t[i + j] = b'*';
                    }
                    res.push(i as i32);
                    is_updated = true;
                }
            }
            if !is_updated {
                break;
            }
        }
        if t.iter().any(|&x| x != b'*') {
            return vec![];
        }
        res.reverse();
        res
    }

    fn can_update(s: &[u8], t: &[u8], i: usize) -> bool {
        let mut flag = false;
        for j in 0..s.len() {
            if t[i + j] == b'*' {
                continue;
            } else if t[i + j] != s[j] {
                flag = false;
                break;
            } else {
                flag = true
            }
        }
        flag
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let stamp = String::from("abc");
        let target = String::from("ababc");
        assert_eq!(Solution::moves_to_stamp(stamp, target), vec![0, 2]);

        let stamp = String::from("abca");
        let target = String::from("aabcaca");
        assert_eq!(Solution::moves_to_stamp(stamp, target), vec![0, 3, 1]);

        let stamp = String::from("mda");
        let target = String::from("mdadddaaaa");
        assert_eq!(Solution::moves_to_stamp(stamp, target), vec![]);
    }
}
