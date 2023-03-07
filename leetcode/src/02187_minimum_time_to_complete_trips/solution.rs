struct Solution;

impl Solution {
    pub fn minimum_time(time: Vec<i32>, total_trips: i32) -> i64 {
        let time = time.into_iter().map(|t| t as i64).collect::<Vec<_>>();
        let total_trips = total_trips as i64;
        let (mut lo, mut hi) = (1, *time.iter().min().unwrap() * total_trips);
        while lo < hi {
            let mid = (hi - lo) / 2 + lo;
            if Self::is_valid(&time, total_trips, mid) {
                hi = mid;
            } else {
                lo = mid + 1;
            }
        }
        lo
    }

    fn is_valid(time: &[i64], total_trips: i64, t: i64) -> bool {
        let mut trips = 0;
        for num in time {
            trips += t / num;
            if trips >= total_trips {
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let time = vec![1, 2, 3];
        let total_trips = 5;
        assert_eq!(Solution::minimum_time(time, total_trips), 3);

        let time = vec![2];
        let total_trips = 1;
        assert_eq!(Solution::minimum_time(time, total_trips), 2);
    }
}
