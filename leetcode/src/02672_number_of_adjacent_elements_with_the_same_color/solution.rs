struct Solution;

impl Solution {
    pub fn color_the_array(n: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let n = n as usize;
        let mut nums = vec![0; n];
        let mut cnt = 0;
        queries
            .into_iter()
            .map(|q| {
                let (i, color) = (q[0] as usize, q[1]);
                if nums[i] > 0 {
                    if i >= 1 && nums[i - 1] == nums[i] {
                        cnt -= 1;
                    }
                    if i + 1 < n && nums[i + 1] == nums[i] {
                        cnt -= 1;
                    }
                }
                nums[i] = color;
                if i >= 1 && nums[i - 1] == nums[i] {
                    cnt += 1;
                }
                if i + 1 < n && nums[i + 1] == nums[i] {
                    cnt += 1;
                }
                cnt
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let n = 4;
        let queries = vec![vec![0, 2], vec![1, 2], vec![3, 1], vec![1, 1], vec![2, 1]];
        assert_eq!(Solution::color_the_array(n, queries), vec![0, 1, 1, 0, 2]);

        let n = 1;
        let queries = vec![vec![0, 100000]];
        assert_eq!(Solution::color_the_array(n, queries), vec![0]);
    }
}
