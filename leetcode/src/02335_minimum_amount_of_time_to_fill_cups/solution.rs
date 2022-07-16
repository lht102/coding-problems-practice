struct Solution;

impl Solution {
    pub fn fill_cups(amount: Vec<i32>) -> i32 {
        let mut amount = amount;
        amount.sort_unstable();
        let total = amount.iter().sum::<i32>();
        if amount[0] + amount[1] > amount[2] {
            total / 2 + total % 2
        } else {
            amount[2]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let amount = vec![1, 4, 2];
        assert_eq!(Solution::fill_cups(amount), 4);

        let amount = vec![5, 4, 4];
        assert_eq!(Solution::fill_cups(amount), 7);
    }
}
