struct Solution;

impl Solution {
    pub fn max_value(n: i32, index: i32, max_sum: i32) -> i32 {
        let n = n as i64;
        let index = index as i64;
        let max_sum = max_sum as i64;
        let mut res = 0;
        let (mut lo, mut hi) = (1, max_sum + 1);
        while lo < hi {
            let mid = (hi - lo) / 2 + lo;
            if Self::is_valid(n, index, mid, max_sum) {
                lo = mid + 1;
                res = mid;
            } else {
                hi = mid;
            }
        }
        res as _
    }

    fn is_valid(n: i64, i: i64, max_val: i64, max_sum: i64) -> bool {
        Self::calculate_i_to_j_sum((max_val - i).max(1), max_val - 1)
            + (i - max_val + 1).max(0)
            + max_val
            + Self::calculate_i_to_j_sum((max_val - (n - i - 1)).max(1), max_val - 1)
            + (n - i - max_val).max(0)
            <= max_sum
    }

    fn calculate_i_to_j_sum(i: i64, j: i64) -> i64 {
        if i > j {
            return 0;
        }
        j * (j + 1) / 2 - (i - 1) * i / 2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let n = 4;
        let index = 2;
        let max_sum = 6;
        assert_eq!(Solution::max_value(n, index, max_sum), 2);

        let n = 6;
        let index = 1;
        let max_sum = 10;
        assert_eq!(Solution::max_value(n, index, max_sum), 3);

        let n = 1;
        let index = 0;
        let max_sum = 24;
        assert_eq!(Solution::max_value(n, index, max_sum), 24);

        let n = 3;
        let index = 0;
        let max_sum = 815094800;
        assert_eq!(Solution::max_value(n, index, max_sum), 271698267);
    }
}
