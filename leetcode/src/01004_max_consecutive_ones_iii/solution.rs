struct Solution;

impl Solution {
    pub fn longest_ones(nums: Vec<i32>, k: i32) -> i32 {
        let psum = std::iter::once(0)
            .chain(nums.iter().scan(0, |acc, num| {
                *acc += num;
                Some(*acc)
            }))
            .collect::<Vec<_>>();
        let mut res = 0;
        let (mut lo, mut hi) = (0, nums.len() + 1);
        while lo < hi {
            let mid = lo + (hi - lo) / 2;
            let mut is_valid = false;
            let (mut i, mut j) = (0, mid);
            while j < psum.len() {
                if psum[j] - psum[i] + k >= mid as i32 {
                    is_valid = true;
                    break;
                }
                i += 1;
                j += 1;
            }
            if is_valid {
                res = mid;
                lo = mid + 1;
            } else {
                hi = mid;
            }
        }
        res as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let nums = vec![1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 0];
        let k = 2;
        assert_eq!(Solution::longest_ones(nums, k), 6);

        let nums = vec![1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 0];
        let k = 3;
        assert_eq!(Solution::longest_ones(nums, k), 10);
    }
}
