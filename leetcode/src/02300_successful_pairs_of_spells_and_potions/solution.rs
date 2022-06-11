struct Solution;

impl Solution {
    pub fn successful_pairs(spells: Vec<i32>, potions: Vec<i32>, success: i64) -> Vec<i32> {
        let mut potions = potions;
        potions.sort_unstable();
        let mut res = Vec::with_capacity(spells.len());
        for &s in &spells {
            let idx = potions.partition_point(|&p| (p as i64 * s as i64) < success);
            res.push((potions.len() - idx) as i32);
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let spells = vec![5, 1, 3];
        let potions = vec![1, 2, 3, 4, 5];
        let success = 7;
        assert_eq!(
            Solution::successful_pairs(spells, potions, success),
            vec![4, 0, 3]
        );

        let spells = vec![3, 1, 2];
        let potions = vec![8, 5, 8];
        let success = 16;
        assert_eq!(
            Solution::successful_pairs(spells, potions, success),
            vec![2, 0, 2]
        );
    }
}
