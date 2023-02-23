use std::collections::BinaryHeap;

struct Solution;

impl Solution {
    pub fn find_maximized_capital(k: i32, w: i32, profits: Vec<i32>, capital: Vec<i32>) -> i32 {
        let mut projects = profits
            .iter()
            .copied()
            .zip(capital.iter().copied())
            .collect::<Vec<_>>();
        projects.sort_unstable_by(|(_, c1), (_, c2)| c1.cmp(c2));
        let mut pq = BinaryHeap::new();
        let mut idx = 0;
        let mut total_capital = w;
        for _ in 0..k {
            while idx < projects.len() && projects[idx].1 <= total_capital {
                pq.push(projects[idx].0);
                idx += 1;
            }
            if let Some(p) = pq.pop() {
                total_capital += p;
            } else {
                break;
            }
        }
        total_capital
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let k = 2;
        let w = 0;
        let profits = vec![1, 2, 3];
        let capital = vec![0, 1, 1];
        assert_eq!(Solution::find_maximized_capital(k, w, profits, capital), 4);

        let k = 3;
        let w = 0;
        let profits = vec![1, 2, 3];
        let capital = vec![0, 1, 2];
        assert_eq!(Solution::find_maximized_capital(k, w, profits, capital), 6);
    }
}
