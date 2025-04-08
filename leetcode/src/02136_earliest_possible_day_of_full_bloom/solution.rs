struct Solution;

impl Solution {
    pub fn earliest_full_bloom(plant_time: Vec<i32>, grow_time: Vec<i32>) -> i32 {
        let mut arr = plant_time
            .into_iter()
            .zip(grow_time)
            .collect::<Vec<_>>();
        arr.sort_unstable_by(|a, b| b.1.cmp(&a.1));
        let mut res = 0;
        let mut cur_p = 0;
        for (p, g) in arr {
            cur_p += p;
            res = res.max(cur_p + g);
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let plant_time = vec![1, 4, 3];
        let grow_time = vec![2, 3, 1];
        assert_eq!(Solution::earliest_full_bloom(plant_time, grow_time), 9);

        let plant_time = vec![1, 2, 3, 2];
        let grow_time = vec![2, 1, 2, 1];
        assert_eq!(Solution::earliest_full_bloom(plant_time, grow_time), 9);

        let plant_time = vec![1];
        let grow_time = vec![1];
        assert_eq!(Solution::earliest_full_bloom(plant_time, grow_time), 2);
    }
}
