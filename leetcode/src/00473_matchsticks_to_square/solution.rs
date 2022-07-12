struct Solution;

impl Solution {
    pub fn makesquare(matchsticks: Vec<i32>) -> bool {
        let sum: i32 = matchsticks.iter().sum();
        if sum % 4 != 0 {
            return false;
        }
        let mut vi = vec![false; matchsticks.len()];
        Solution::backtracking(&matchsticks, &mut vi, sum / 4, 0, 0, 4)
    }

    fn backtracking(
        arr: &[i32],
        vi: &mut [bool],
        target: i32,
        cur_sum: i32,
        j: usize,
        k: usize,
    ) -> bool {
        if k == 1 {
            return true;
        }
        if cur_sum == target {
            return Solution::backtracking(arr, vi, target, 0, 0, k - 1);
        }
        for i in j..arr.len() {
            if vi[i] || cur_sum + arr[i] > target {
                continue;
            }
            vi[i] = true;
            if Solution::backtracking(arr, vi, target, cur_sum + arr[i], i + 1, k) {
                return true;
            }
            vi[i] = false;
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let matchsticks = vec![1, 1, 2, 2, 2];
        assert!(Solution::makesquare(matchsticks));

        let matchsticks = vec![3, 3, 3, 3, 4];
        assert!(!Solution::makesquare(matchsticks));
    }
}
