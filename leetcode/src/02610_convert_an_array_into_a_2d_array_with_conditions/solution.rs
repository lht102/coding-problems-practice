struct Solution;

impl Solution {
    pub fn find_matrix(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let freq = nums.iter().fold(vec![0; nums.len() + 1], |mut freq, &num| {
            freq[num as usize] += 1;
            freq
        });
        freq.iter().enumerate().fold(
            vec![vec![]; *freq.iter().max().unwrap()],
            |mut res, (num, &cnt)| {
                res.iter_mut().take(cnt).for_each(|row| row.push(num as _));
                res
            },
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let nums = vec![1, 3, 4, 1, 2, 3, 1];
        assert_eq!(
            Solution::find_matrix(nums),
            vec![vec![1, 2, 3, 4], vec![1, 3], vec![1]]
        );

        let nums = vec![1, 2, 3, 4];
        assert_eq!(Solution::find_matrix(nums), vec![vec![1, 2, 3, 4]],);
    }
}
