struct Solution;

impl Solution {
    pub fn find_kth_positive(arr: Vec<i32>, k: i32) -> i32 {
        let (mut lo, mut hi) = (0, arr.len());
        while lo < hi {
            let mid = (hi - lo) / 2 + lo;
            if arr[mid] - mid as i32 - 1 < k {
                lo = mid + 1;
            } else {
                hi = mid;
            }
        }
        lo as i32 + k
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let arr = vec![2, 3, 4, 7, 11];
        let k = 5;
        assert_eq!(Solution::find_kth_positive(arr, k), 9);

        let arr = vec![1, 2, 3, 4];
        let k = 2;
        assert_eq!(Solution::find_kth_positive(arr, k), 6);
    }
}
