struct Solution;

impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let a = a.as_bytes();
        let b = b.as_bytes();
        let mut res = vec![];
        let (mut i, mut j) = (a.len() as i32 - 1, b.len() as i32 - 1);
        let mut inc = 0;
        while i >= 0 || j >= 0 {
            let mut sum = inc;
            if i >= 0 {
                sum += a[i as usize] - b'0';
                i -= 1;
            }
            if j >= 0 {
                sum += b[j as usize] - b'0';
                j -= 1;
            }
            inc = if sum > 1 { 1 } else { 0 };
            res.push(sum % 2 + b'0');
        }
        if inc > 0 {
            res.push(b'1');
        }
        res.reverse();
        String::from_utf8(res).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let a = String::from("11");
        let b = String::from("1");
        assert_eq!(Solution::add_binary(a, b), String::from("100"));

        let a = String::from("1010");
        let b = String::from("1011");
        assert_eq!(Solution::add_binary(a, b), String::from("10101"));
    }
}
