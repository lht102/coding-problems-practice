struct Solution;

impl Solution {
    pub fn fib(n: i32) -> i32 {
        if n < 2 {
            return n;
        }
        let mut prev = 0;
        let mut cur = 1;
        for _ in 2..=n {
            let tmp = cur;
            cur = cur + prev;
            prev = tmp;
        }
        cur
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let n = 2;
        assert_eq!(Solution::fib(n), 1);

        let n = 3;
        assert_eq!(Solution::fib(n), 2);

        let n = 4;
        assert_eq!(Solution::fib(n), 3);
    }
}
