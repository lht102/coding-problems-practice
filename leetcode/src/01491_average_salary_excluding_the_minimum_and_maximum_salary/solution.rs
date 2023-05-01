struct Solution;

impl Solution {
    pub fn average(salary: Vec<i32>) -> f64 {
        (salary.iter().map(|&x| x as f64).sum::<f64>()
            - *salary.iter().min().unwrap() as f64
            - *salary.iter().max().unwrap() as f64)
            / (salary.len() - 2) as f64
            * 100000.0
            / 100000.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let salary = vec![4000, 3000, 1000, 2000];
        assert_eq!(Solution::average(salary), 2500.0);

        let salary = vec![1000, 2000, 3000];
        assert_eq!(Solution::average(salary), 2000.0);

        let salary = vec![8000, 9000, 2000, 3000, 6000, 1000];
        assert_eq!(Solution::average(salary), 4750.0);
    }
}
