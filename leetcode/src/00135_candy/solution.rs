struct Solution;

impl Solution {
    pub fn candy(ratings: Vec<i32>) -> i32 {
        let n = ratings.len();
        let mut candies = vec![1; n];
        for i in 1..n {
            if ratings[i] > ratings[i - 1] {
                candies[i] = candies[i - 1] + 1;
            }
        }
        for i in (1..n).rev() {
            if ratings[i - 1] > ratings[i] {
                candies[i - 1] = candies[i - 1].max(candies[i] + 1);
            }
        }
        candies.iter().sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let ratings = vec![1, 0, 2];
        assert_eq!(Solution::candy(ratings), 5);

        let ratings = vec![1, 2, 2];
        assert_eq!(Solution::candy(ratings), 4);

        let ratings = vec![1, 3, 2, 2, 1];
        assert_eq!(Solution::candy(ratings), 7);
    }
}
