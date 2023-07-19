struct Solution;

impl Solution {
    pub fn erase_overlap_intervals(intervals: Vec<Vec<i32>>) -> i32 {
        if intervals.len() < 2 {
            return 0;
        }
        let mut intervals = intervals;
        intervals.sort_by(|a, b| a[1].cmp(&b[1]));
        let mut left = 0;
        let mut res = 0;
        for right in 1..intervals.len() {
            if intervals[left][1] > intervals[right][0] {
                res += 1;
            } else {
                left = right;
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
        let intervals = vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![1, 3]];
        assert_eq!(Solution::erase_overlap_intervals(intervals), 1);

        let intervals = vec![vec![1, 2], vec![1, 2], vec![1, 2]];
        assert_eq!(Solution::erase_overlap_intervals(intervals), 2);

        let intervals = vec![vec![1, 2], vec![2, 3]];
        assert_eq!(Solution::erase_overlap_intervals(intervals), 0);

        let intervals = vec![
            vec![-52, 31],
            vec![-73, -26],
            vec![82, 97],
            vec![-65, -11],
            vec![-62, -49],
            vec![95, 99],
            vec![58, 95],
            vec![-31, 49],
            vec![66, 98],
            vec![-63, 2],
            vec![30, 47],
            vec![-40, -26],
        ];
        assert_eq!(Solution::erase_overlap_intervals(intervals), 7);

        let intervals = vec![vec![1, 100], vec![11, 22], vec![1, 11], vec![2, 12]];
        assert_eq!(Solution::erase_overlap_intervals(intervals), 2);
    }
}
