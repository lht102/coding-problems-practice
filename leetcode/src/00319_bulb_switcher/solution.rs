struct Solution;

impl Solution {
    pub fn bulb_switch(n: i32) -> i32 {
        (n as f64).sqrt() as _
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let n = 3;
        assert_eq!(Solution::bulb_switch(n), 1);

        let n = 0;
        assert_eq!(Solution::bulb_switch(n), 0);

        let n = 1;
        assert_eq!(Solution::bulb_switch(n), 1);
    }
}
