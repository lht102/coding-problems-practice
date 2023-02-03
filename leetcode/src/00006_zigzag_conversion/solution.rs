struct Solution;

impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows <= 1 {
            return s;
        }
        let num_rows = num_rows as usize;
        let mut rows = vec![vec![]; num_rows];
        let mut i = 0;
        let mut down = true;
        for ch in s.chars() {
            rows[i].push(ch);
            if i == 0 || i == num_rows - 1 {
                down = !down;
            }
            if down {
                i -= 1;
            } else {
                i += 1;
            }
        }
        rows.into_iter().flatten().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let s = String::from("PAYPALISHIRING");
        let num_rows = 3;
        assert_eq!(
            Solution::convert(s, num_rows),
            String::from("PAHNAPLSIIGYIR")
        );

        let s = String::from("PAYPALISHIRING");
        let num_rows = 4;
        assert_eq!(
            Solution::convert(s, num_rows),
            String::from("PINALSIGYAHRPI")
        );

        let s = String::from("A");
        let num_rows = 1;
        assert_eq!(Solution::convert(s, num_rows), String::from("A"));
    }
}
