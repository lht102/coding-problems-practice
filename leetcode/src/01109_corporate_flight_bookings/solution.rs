struct Solution;

impl Solution {
    pub fn corp_flight_bookings(bookings: Vec<Vec<i32>>, n: i32) -> Vec<i32> {
        let n = n as usize;
        let mut diffs = vec![0; n + 1];
        for booking in &bookings {
            let (left, right) = (booking[0] as usize - 1, booking[1] as usize - 1);
            diffs[left] += booking[2];
            diffs[right + 1] -= booking[2];
        }
        diffs[0..n]
            .iter()
            .scan(0, |acc, num| {
                *acc += num;
                Some(*acc)
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let bookings = vec![vec![1, 2, 10], vec![2, 3, 20], vec![2, 5, 25]];
        let n = 5;
        assert_eq!(
            Solution::corp_flight_bookings(bookings, n),
            vec![10, 55, 45, 25, 25]
        );

        let bookings = vec![vec![1, 2, 10], vec![2, 2, 15]];
        let n = 2;
        assert_eq!(Solution::corp_flight_bookings(bookings, n), vec![10, 25]);
    }
}
