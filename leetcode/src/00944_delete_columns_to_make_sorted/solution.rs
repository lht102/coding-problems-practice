struct Solution;

impl Solution {
    pub fn min_deletion_size(strs: Vec<String>) -> i32 {
        let m = strs.len();
        let n = strs[0].len();
        let mut mat = Vec::with_capacity(m);
        for s in &strs {
            mat.push(s.chars().collect::<Vec<_>>());
        }
        let mut res = 0;
        for i in 0..n {
            for j in 1..m {
                if mat[j - 1][i] > mat[j][i] {
                    res += 1;
                    break;
                }
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let strs = vec![
            String::from("cba"),
            String::from("daf"),
            String::from("ghi"),
        ];
        assert_eq!(Solution::min_deletion_size(strs), 1);

        let strs = vec![String::from("a"), String::from("b")];
        assert_eq!(Solution::min_deletion_size(strs), 0);

        let strs = vec![
            String::from("zyx"),
            String::from("wvu"),
            String::from("tsr"),
        ];
        assert_eq!(Solution::min_deletion_size(strs), 3);
    }
}
