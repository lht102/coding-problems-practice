struct Solution;

impl Solution {
    pub fn check_straight_line(coordinates: Vec<Vec<i32>>) -> bool {
        let first_line_val = Self::line_val(&coordinates[0], &coordinates[1]);
        for coordinate in coordinates.iter().skip(1) {
            if Self::line_val(coordinate, &coordinates[0]) != first_line_val {
                return false;
            }
        }
        true
    }

    fn line_val(c1: &[i32], c2: &[i32]) -> (i32, i32) {
        let dx = c2[0] - c1[0];
        let dy = c2[1] - c1[1];
        let d = Self::gcd(dx, dy);
        (dx / d, dy / d)
    }

    fn gcd(a: i32, b: i32) -> i32 {
        if a == 0 {
            b
        } else {
            Self::gcd(b % a, a)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let coordinates = vec![
            vec![1, 2],
            vec![2, 3],
            vec![3, 4],
            vec![4, 5],
            vec![5, 6],
            vec![6, 7],
        ];
        assert!(Solution::check_straight_line(coordinates));

        let coordinates = vec![
            vec![1, 1],
            vec![2, 2],
            vec![3, 4],
            vec![4, 5],
            vec![5, 6],
            vec![7, 7],
        ];
        assert!(!Solution::check_straight_line(coordinates));
    }
}
