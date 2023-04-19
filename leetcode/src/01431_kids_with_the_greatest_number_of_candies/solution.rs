struct Solution;

impl Solution {
    pub fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
        let max_c = *candies.iter().max().unwrap();
        candies
            .into_iter()
            .map(|x| (x + extra_candies) >= max_c)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let candies = vec![2, 3, 5, 1, 3];
        let extra_candies = 3;
        assert_eq!(
            Solution::kids_with_candies(candies, extra_candies),
            vec![true, true, true, false, true]
        );

        let candies = vec![4, 2, 1, 1, 2];
        let extra_candies = 1;
        assert_eq!(
            Solution::kids_with_candies(candies, extra_candies),
            vec![true, false, false, false, false]
        );

        let candies = vec![12, 1, 12];
        let extra_candies = 10;
        assert_eq!(
            Solution::kids_with_candies(candies, extra_candies),
            vec![true, false, true]
        );
    }
}
