struct Solution;

impl Solution {
    pub fn max_area(h: i32, w: i32, horizontal_cuts: Vec<i32>, vertical_cuts: Vec<i32>) -> i32 {
        let max_gap = |cuts: &mut Vec<i32>, sz| {
            cuts.extend([0, sz]);
            cuts.sort_unstable();
            cuts.windows(2).map(|w| w[1] - w[0]).max().unwrap() as u64
        };
        let mut hc = horizontal_cuts;
        let mut vc = vertical_cuts;
        (max_gap(&mut hc, h) * max_gap(&mut vc, w) % 1000000007) as _
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let h = 5;
        let w = 4;
        let horizontal_cuts = vec![1, 2, 4];
        let vertical_cuts = vec![1, 3];
        assert_eq!(Solution::max_area(h, w, horizontal_cuts, vertical_cuts), 4);

        let h = 5;
        let w = 4;
        let horizontal_cuts = vec![3, 1];
        let vertical_cuts = vec![1];
        assert_eq!(Solution::max_area(h, w, horizontal_cuts, vertical_cuts), 6);

        let h = 5;
        let w = 4;
        let horizontal_cuts = vec![3];
        let vertical_cuts = vec![3];
        assert_eq!(Solution::max_area(h, w, horizontal_cuts, vertical_cuts), 9);
    }
}
