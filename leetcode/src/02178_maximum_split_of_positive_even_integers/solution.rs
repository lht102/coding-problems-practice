struct Solution;

impl Solution {
    pub fn maximum_even_split(final_sum: i64) -> Vec<i64> {
        if final_sum % 2 == 1 {
            return vec![];
        }
        let mut res = Vec::new();
        let mut final_sum = final_sum;
        let mut num = 2;
        while final_sum >= num {
            final_sum -= num;
            res.push(num);
            num += 2;
        }
        *res.last_mut().unwrap() += final_sum;
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let final_sum = 12;
        assert_eq!(Solution::maximum_even_split(final_sum), vec![2, 4, 6]);

        let final_sum = 7;
        assert_eq!(Solution::maximum_even_split(final_sum), vec![]);

        let final_sum = 28;
        assert_eq!(Solution::maximum_even_split(final_sum), vec![2, 4, 6, 16]);
    }
}
