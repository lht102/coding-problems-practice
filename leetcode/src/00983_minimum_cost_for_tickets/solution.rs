use std::collections::VecDeque;

struct Solution;

impl Solution {
    pub fn mincost_tickets(days: Vec<i32>, costs: Vec<i32>) -> i32 {
        let passes = [1, 7, 30];
        let mut queues: Vec<VecDeque<(i32, i32)>> = vec![VecDeque::new(); passes.len()];
        let mut res = 0;
        for day in days {
            for (i, &pass) in passes.iter().enumerate() {
                while !queues[i].is_empty() && queues[i][0].0 + pass <= day {
                    queues[i].pop_front();
                }
                queues[i].push_back((day, res + costs[i]));
            }
            res = queues.iter().map(|x| x[0].1).min().unwrap();
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let days = vec![1, 4, 6, 7, 8, 20];
        let costs = vec![2, 7, 15];
        assert_eq!(Solution::mincost_tickets(days, costs), 11);

        let days = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 30, 31];
        let costs = vec![2, 7, 15];
        assert_eq!(Solution::mincost_tickets(days, costs), 17);
    }
}
