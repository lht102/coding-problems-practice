use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn tallest_billboard(rods: Vec<i32>) -> i32 {
        Self::solve(&rods, 0, 0, &mut HashMap::new())
    }

    fn solve(rods: &[i32], i: usize, diff: i32, dp: &mut HashMap<(usize, i32), i32>) -> i32 {
        if i >= rods.len() {
            if diff == 0 {
                return 0;
            }
            return i32::MIN;
        }
        if let Some(&val) = dp.get(&(i, diff)) {
            return val;
        }
        let s1 = Self::solve(rods, i + 1, diff + rods[i], dp) + rods[i];
        let s2 = Self::solve(rods, i + 1, diff - rods[i], dp);
        let no_s = Self::solve(rods, i + 1, diff, dp);
        let res = s1.max(s2).max(no_s);
        dp.insert((i, diff), res);
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let rods = vec![1, 2, 3, 6];
        assert_eq!(Solution::tallest_billboard(rods), 6);

        let rods = vec![1, 2, 3, 4, 5, 6];
        assert_eq!(Solution::tallest_billboard(rods), 10);

        let rods = vec![1, 2];
        assert_eq!(Solution::tallest_billboard(rods), 0);
    }
}
