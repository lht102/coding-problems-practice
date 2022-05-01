struct Solution;

impl Solution {
    pub fn min_cost(
        start_pos: Vec<i32>,
        home_pos: Vec<i32>,
        row_costs: Vec<i32>,
        col_costs: Vec<i32>,
    ) -> i32 {
        let diff = (home_pos[0] - start_pos[0], home_pos[1] - start_pos[1]);
        let mut res = 0;
        if diff.0 < 0 {
            for i in home_pos[0]..=start_pos[0] - 1 {
                res += row_costs[i as usize];
            }
        } else {
            for i in start_pos[0] + 1..=home_pos[0] {
                res += row_costs[i as usize];
            }
        }
        if diff.1 < 0 {
            for i in home_pos[1]..=start_pos[1] - 1 {
                res += col_costs[i as usize];
            }
        } else {
            for i in start_pos[1] + 1..=home_pos[1] {
                res += col_costs[i as usize];
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let start_pos = vec![1, 0];
        let home_pos = vec![2, 3];
        let row_costs = vec![5, 4, 3];
        let col_costs = vec![8, 2, 6, 7];
        assert_eq!(
            Solution::min_cost(start_pos, home_pos, row_costs, col_costs),
            18
        );

        let start_pos = vec![0, 0];
        let home_pos = vec![0, 0];
        let row_costs = vec![5];
        let col_costs = vec![26];
        assert_eq!(
            Solution::min_cost(start_pos, home_pos, row_costs, col_costs),
            0
        );
    }
}
