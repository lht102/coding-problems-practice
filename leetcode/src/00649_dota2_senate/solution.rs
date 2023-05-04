use std::collections::VecDeque;

struct Solution;

impl Solution {
    pub fn predict_party_victory(senate: String) -> String {
        let n = senate.len();
        let (mut qr, mut qd) = (VecDeque::new(), VecDeque::new());
        for (i, ch) in senate.char_indices() {
            if ch == 'R' {
                qr.push_back(i);
            } else {
                qd.push_back(i);
            }
        }
        while !qr.is_empty() && !qd.is_empty() {
            let r = qr.pop_front().unwrap();
            let d = qd.pop_front().unwrap();
            if r < d {
                qr.push_back(r + n);
            } else {
                qd.push_back(d + n);
            }
        }
        if qr.len() < qd.len() {
            String::from("Dire")
        } else {
            String::from("Radiant")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let senate = String::from("RD");
        assert_eq!(
            Solution::predict_party_victory(senate),
            String::from("Radiant")
        );

        let senate = String::from("RDD");
        assert_eq!(
            Solution::predict_party_victory(senate),
            String::from("Dire")
        );
    }
}
