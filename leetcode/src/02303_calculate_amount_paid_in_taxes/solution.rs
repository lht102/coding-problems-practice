struct Solution;

impl Solution {
    pub fn calculate_tax(brackets: Vec<Vec<i32>>, income: i32) -> f64 {
        let mut res = 0.0;
        let mut prev = 0;
        for b in &brackets {
            let (upper, percent) = (b[0], b[1] as f64);
            if income >= upper {
                res += (upper - prev) as f64 * percent / 100.0;
                prev = upper;
            } else {
                res += (income - prev) as f64 * percent / 100.0;
                break;
            }
        }
        (res * 100000.0).round() / 100000.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let brackets = vec![vec![3, 50], vec![7, 10], vec![12, 25]];
        let income = 10;
        assert_eq!(Solution::calculate_tax(brackets, income), 2.65);

        let brackets = vec![vec![1, 0], vec![4, 25], vec![5, 50]];
        let income = 2;
        assert_eq!(Solution::calculate_tax(brackets, income), 0.25);

        let brackets = vec![vec![2, 50]];
        let income = 0;
        assert_eq!(Solution::calculate_tax(brackets, income), 0.0);
    }
}
