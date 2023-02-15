struct Solution;

impl Solution {
    pub fn add_to_array_form(num: Vec<i32>, k: i32) -> Vec<i32> {
        let mut res = num;
        let mut t = k;
        for n in res.iter_mut().rev() {
            *n += t;
            t = *n / 10;
            *n %= 10;
        }
        res.reverse();
        while t > 0 {
            res.push(t % 10);
            t /= 10;
        }
        res.reverse();
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let num = vec![1, 2, 0, 0];
        let k = 34;
        assert_eq!(Solution::add_to_array_form(num, k), vec![1, 2, 3, 4]);

        let num = vec![2, 7, 4];
        let k = 181;
        assert_eq!(Solution::add_to_array_form(num, k), vec![4, 5, 5]);

        let num = vec![2, 1, 5];
        let k = 806;
        assert_eq!(Solution::add_to_array_form(num, k), vec![1, 0, 2, 1]);
    }
}
