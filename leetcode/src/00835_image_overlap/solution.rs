use std::collections::HashMap;
use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn largest_overlap(img1: Vec<Vec<i32>>, img2: Vec<Vec<i32>>) -> i32 {
        let mut set1 = HashSet::new();
        let mut set2 = HashSet::new();
        for i in 0..img1.len() {
            for j in 0..img1[0].len() {
                if img1[i][j] == 1 {
                    set1.insert((i as i32, j as i32));
                }
                if img2[i][j] == 1 {
                    set2.insert((i as i32, j as i32));
                }
            }
        }
        let mut res = 0;
        let mut freq = HashMap::new();
        for (y1, x1) in set1.iter() {
            for (y2, x2) in set2.iter() {
                let cnt = freq.entry((y2 - y1, x2 - x1)).or_default();
                *cnt += 1;
                res = res.max(*cnt);
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
        let img1 = vec![vec![1, 1, 0], vec![0, 1, 0], vec![0, 1, 0]];
        let img2 = vec![vec![0, 0, 0], vec![0, 1, 1], vec![0, 0, 1]];
        assert_eq!(Solution::largest_overlap(img1, img2), 3);

        let img1 = vec![vec![1]];
        let img2 = vec![vec![1]];
        assert_eq!(Solution::largest_overlap(img1, img2), 1);

        let img1 = vec![vec![0]];
        let img2 = vec![vec![0]];
        assert_eq!(Solution::largest_overlap(img1, img2), 0);
    }
}
