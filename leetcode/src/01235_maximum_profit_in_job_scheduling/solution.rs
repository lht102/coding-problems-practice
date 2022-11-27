use std::collections::BTreeMap;

struct Solution;

impl Solution {
    pub fn job_scheduling(start_time: Vec<i32>, end_time: Vec<i32>, profit: Vec<i32>) -> i32 {
        let mut jobs = (0..start_time.len())
            .map(|i| (end_time[i], start_time[i], profit[i]))
            .collect::<Vec<_>>();
        jobs.sort_unstable_by(|a, b| a.0.cmp(&b.0));
        let mut dp = BTreeMap::from([(0, 0)]);
        for (end_t, start_t, p) in jobs {
            let current_profit = dp.range(..=start_t).rev().next().unwrap().1 + p;
            if current_profit > *dp.iter().rev().next().unwrap().1 {
                dp.insert(end_t, current_profit);
            }
        }
        *dp.iter().rev().next().unwrap().1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let start_time = vec![1, 2, 3, 3];
        let end_time = vec![3, 4, 5, 6];
        let profit = vec![50, 10, 40, 70];
        assert_eq!(Solution::job_scheduling(start_time, end_time, profit), 120);

        let start_time = vec![1, 2, 3, 4, 6];
        let end_time = vec![3, 5, 10, 6, 9];
        let profit = vec![20, 20, 100, 70, 60];
        assert_eq!(Solution::job_scheduling(start_time, end_time, profit), 150);

        let start_time = vec![1, 1, 1];
        let end_time = vec![2, 3, 4];
        let profit = vec![5, 6, 4];
        assert_eq!(Solution::job_scheduling(start_time, end_time, profit), 6);
    }
}
