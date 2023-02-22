struct Solution;

impl Solution {
    pub fn ship_within_days(weights: Vec<i32>, days: i32) -> i32 {
        let (mut lo, mut hi) = (*weights.iter().max().unwrap(), weights.iter().sum());
        while lo < hi {
            let mid = (hi - lo) / 2 + lo;
            if Self::is_valid(&weights, days, mid as _) {
                hi = mid;
            } else {
                lo = mid + 1;
            }
        }
        lo as _
    }

    fn is_valid(weights: &[i32], days: i32, capacity: i32) -> bool {
        let mut required_days = 1;
        let mut cur_weight = 0;
        for &weight in weights {
            cur_weight += weight;
            if cur_weight > capacity {
                required_days += 1;
                cur_weight = weight;
                if required_days > days {
                    return false;
                }
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let weights = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let days = 5;
        assert_eq!(Solution::ship_within_days(weights, days), 15);

        let weights = vec![3, 2, 2, 4, 1, 4];
        let days = 3;
        assert_eq!(Solution::ship_within_days(weights, days), 6);

        let weights = vec![1, 2, 3, 1, 1];
        let days = 4;
        assert_eq!(Solution::ship_within_days(weights, days), 3);
    }
}
