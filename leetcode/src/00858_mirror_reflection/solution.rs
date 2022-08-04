struct Solution;

impl Solution {
    pub fn mirror_reflection(p: i32, q: i32) -> i32 {
        let lcm = p * q / Solution::gcd(p, q);
        let m = lcm / p;
        let n = lcm / q;
        match (m % 2, n % 2) {
            (0, 1) => 0,
            (1, 1) => 1,
            (1, 0) => 2,
            _ => -1,
        }
    }

    fn gcd(a: i32, b: i32) -> i32 {
        let mut a = a;
        let mut b = b;
        while b != 0 {
            a %= b;
            let tmp = b;
            b = a;
            a = tmp;
        }
        a
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let p = 2;
        let q = 1;
        assert_eq!(Solution::mirror_reflection(p, q), 2);

        let p = 3;
        let q = 1;
        assert_eq!(Solution::mirror_reflection(p, q), 1);
    }
}
