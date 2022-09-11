use std::cmp::Reverse;
use std::collections::BinaryHeap;

struct Solution;

impl Solution {
    pub fn max_performance(_n: i32, speed: Vec<i32>, efficiency: Vec<i32>, k: i32) -> i32 {
        let mut arr = efficiency
            .into_iter()
            .map(|x| x as i64)
            .zip(speed.into_iter().map(|x| x as i64))
            .collect::<Vec<_>>();
        arr.sort_unstable_by_key(|k| Reverse(*k));
        let mut res = 0;
        let mut total_speed = 0;
        let mut pq = BinaryHeap::new();
        let k = k as usize;
        for (e, s) in arr {
            pq.push(Reverse(s));
            total_speed += s;
            if pq.len() > k {
                total_speed -= pq.pop().unwrap().0;
            }
            res = res.max(total_speed * e);
        }
        (res % 1000000007) as _
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let n = 6;
        let speed = vec![2, 10, 3, 1, 5, 8];
        let efficiency = vec![5, 4, 3, 9, 7, 2];
        let k = 2;
        assert_eq!(Solution::max_performance(n, speed, efficiency, k), 60);

        let n = 6;
        let speed = vec![2, 10, 3, 1, 5, 8];
        let efficiency = vec![5, 4, 3, 9, 7, 2];
        let k = 3;
        assert_eq!(Solution::max_performance(n, speed, efficiency, k), 68);

        let n = 6;
        let speed = vec![2, 10, 3, 1, 5, 8];
        let efficiency = vec![5, 4, 3, 9, 7, 2];
        let k = 4;
        assert_eq!(Solution::max_performance(n, speed, efficiency, k), 72);
    }
}
