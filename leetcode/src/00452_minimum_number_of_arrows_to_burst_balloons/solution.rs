struct Solution;

impl Solution {
    pub fn find_min_arrow_shots(points: Vec<Vec<i32>>) -> i32 {
        let mut points = points;
        points.sort_unstable_by(|a, b| a[1].cmp(&b[1]));
        let mut res = 1;
        let mut cur = points[0][1];
        for point in &points {
            if cur < point[0] {
                res += 1;
                cur = point[1];
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
        let points = vec![vec![10, 16], vec![2, 8], vec![1, 6], vec![7, 12]];
        assert_eq!(Solution::find_min_arrow_shots(points), 2);

        let points = vec![vec![10, 16], vec![2, 8], vec![1, 6], vec![7, 12]];
        assert_eq!(Solution::find_min_arrow_shots(points), 2);
    }
}
