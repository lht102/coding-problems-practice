struct Solution;

impl Solution {
    pub fn minimum_lines(stock_prices: Vec<Vec<i32>>) -> i32 {
        if stock_prices.len() == 1 {
            return 0;
        }
        let mut sps = stock_prices;
        sps.sort_by(|a, b| a[0].cmp(&b[0]));
        let mut res = 1;
        for i in 2..sps.len() {
            let (x0, y0) = (sps[i - 2][0] as i64, sps[i - 2][1] as i64);
            let (x1, y1) = (sps[i - 1][0] as i64, sps[i - 1][1] as i64);
            let (x2, y2) = (sps[i][0] as i64, sps[i][1] as i64);
            if (y2 - y1) * (x1 - x0) != (y1 - y0) * (x2 - x1) {
                res += 1;
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
        let stock_prices = vec![
            vec![1, 7],
            vec![2, 6],
            vec![3, 5],
            vec![4, 4],
            vec![5, 4],
            vec![6, 3],
            vec![7, 2],
            vec![8, 1],
        ];
        assert_eq!(Solution::minimum_lines(stock_prices), 3);

        let stock_prices = vec![vec![3, 4], vec![1, 2], vec![7, 8], vec![2, 3]];
        assert_eq!(Solution::minimum_lines(stock_prices), 1);
    }
}
