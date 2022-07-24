struct Solution;

impl Solution {
    pub fn count_smaller(nums: Vec<i32>) -> Vec<i32> {
        let mut arr: Vec<_> = nums.into_iter().enumerate().map(|(i, v)| (v, i)).collect();
        let mut res = vec![0; arr.len()];
        Solution::count_inversion(&mut arr, &mut res);
        res
    }

    fn merge<T: Ord + Copy>(arr: &mut [(T, usize)], mid: usize, res: &mut [i32]) {
        let left_half = arr[..mid].to_vec();
        let right_half = arr[mid..].to_vec();
        let mut i = 0;
        let mut j = 0;
        let mut k = 0;
        let mut cnt = 0;
        while i < left_half.len() && j < right_half.len() {
            if left_half[i].0 > right_half[j].0 {
                arr[k] = right_half[j];
                cnt += 1;
                j += 1;
            } else {
                arr[k] = left_half[i];
                res[left_half[i].1] += cnt;
                i += 1;
            }
            k += 1;
        }
        for t in i..left_half.len() {
            arr[k] = left_half[t];
            res[left_half[t].1] += cnt;
            k += 1;
        }
        for &item in right_half.iter().skip(j) {
            arr[k] = item;
            k += 1;
        }
    }

    fn count_inversion<T: Ord + Copy>(arr: &mut [(T, usize)], res: &mut [i32]) {
        if arr.len() > 1 {
            let mid = arr.len() / 2;
            Solution::count_inversion(&mut arr[..mid], res);
            Solution::count_inversion(&mut arr[mid..], res);
            Solution::merge(arr, mid, res)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let nums = vec![5, 2, 6, 1];
        assert_eq!(Solution::count_smaller(nums), vec![2, 1, 1, 0]);

        let nums = vec![-1];
        assert_eq!(Solution::count_smaller(nums), vec![0]);

        let nums = vec![-1, -1];
        assert_eq!(Solution::count_smaller(nums), vec![0, 0]);
    }
}
