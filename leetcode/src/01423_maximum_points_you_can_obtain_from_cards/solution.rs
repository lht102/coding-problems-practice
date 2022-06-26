struct Solution;

impl Solution {
    pub fn max_score(card_points: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let psum = std::iter::once(0)
            .chain(card_points.iter().take(k).scan(0, |acc, num| {
                *acc += num;
                Some(*acc)
            }))
            .collect::<Vec<_>>();
        let ssum = std::iter::once(0)
            .chain(card_points.iter().rev().take(k).scan(0, |acc, num| {
                *acc += num;
                Some(*acc)
            }))
            .collect::<Vec<_>>();
        let mut res = 0;
        for i in 0..=card_points.len().min(k) {
            res = res.max(psum[i] + ssum[k - i]);
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let card_points = vec![1, 2, 3, 4, 5, 6, 1];
        let k = 3;
        assert_eq!(Solution::max_score(card_points, k), 12);

        let card_points = vec![2, 2, 2];
        let k = 2;
        assert_eq!(Solution::max_score(card_points, k), 4);

        let card_points = vec![9, 7, 7, 9, 7, 7, 9];
        let k = 7;
        assert_eq!(Solution::max_score(card_points, k), 55);
    }
}
