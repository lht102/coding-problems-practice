struct Solution;

impl Solution {
    pub fn poor_pigs(buckets: i32, minutes_to_die: i32, minutes_to_test: i32) -> i32 {
        let mut res = 0;
        let t = minutes_to_test / minutes_to_die;
        while (t + 1).pow(res) < buckets {
            res += 1;
        }
        res as _
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let buckets = 1000;
        let minutes_to_die = 15;
        let minutes_to_test = 60;
        assert_eq!(
            Solution::poor_pigs(buckets, minutes_to_die, minutes_to_test),
            5
        );

        let buckets = 4;
        let minutes_to_die = 15;
        let minutes_to_test = 15;
        assert_eq!(
            Solution::poor_pigs(buckets, minutes_to_die, minutes_to_test),
            2
        );

        let buckets = 4;
        let minutes_to_die = 15;
        let minutes_to_test = 30;
        assert_eq!(
            Solution::poor_pigs(buckets, minutes_to_die, minutes_to_test),
            2
        );
    }
}
