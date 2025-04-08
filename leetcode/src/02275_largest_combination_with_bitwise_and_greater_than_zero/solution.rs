struct Solution;

impl Solution {
    pub fn largest_combination(candidates: Vec<i32>) -> i32 {
        let mut freq = [0; 32];
        let mut candidates = candidates;
        candidates.sort_unstable();
        for &num in &candidates {
            for (pow, item) in freq.iter_mut().enumerate().take(32) {
                if num & (1 << pow) > 0 {
                    *item += 1;
                }
            }
        }
        *freq.iter().max().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let candidates = vec![16, 17, 71, 62, 12, 24, 14];
        assert_eq!(Solution::largest_combination(candidates), 4);

        let candidates = vec![8, 8];
        assert_eq!(Solution::largest_combination(candidates), 2);
    }
}
