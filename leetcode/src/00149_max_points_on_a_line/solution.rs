use std::collections::HashMap;
use std::hash::Hash;
use std::hash::Hasher;

struct Solution;

struct NonNan(f64);

impl NonNan {
    fn key(&self) -> u64 {
        self.0.to_bits()
    }
}

impl PartialEq for NonNan {
    fn eq(&self, other: &Self) -> bool {
        self.key() == other.key()
    }
}

impl Eq for NonNan {}

impl Hash for NonNan {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.key().hash(state)
    }
}

impl Solution {
    pub fn max_points(points: Vec<Vec<i32>>) -> i32 {
        let n = points.len();
        if n == 1 {
            return 1;
        }
        let mut res = 2;
        for i in 0..n {
            let mut freq = HashMap::<NonNan, usize>::new();
            for j in 0..n {
                if i != j {
                    let y = (points[j][1] - points[i][1]) as f64;
                    let x = (points[j][0] - points[i][0]) as f64;
                    *freq.entry(NonNan(y.atan2(x))).or_default() += 1;
                }
            }
            res = res.max(*freq.values().max().unwrap() + 1);
        }
        res as _
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let points = vec![vec![1, 1], vec![2, 2], vec![3, 3]];
        assert_eq!(Solution::max_points(points), 3);

        let points = vec![
            vec![1, 1],
            vec![3, 2],
            vec![5, 3],
            vec![4, 1],
            vec![2, 3],
            vec![1, 4],
        ];
        assert_eq!(Solution::max_points(points), 4);
    }
}
