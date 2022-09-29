struct Solution;

impl Solution {
    pub fn find_closest_elements(arr: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
        let k = k as usize;
        let (mut lo, mut hi) = (0, arr.len() - k);
        while lo < hi {
            let mid = lo + (hi - lo) / 2;
            if x - arr[mid] > arr[mid + k] - x {
                lo = mid + 1;
            } else {
                hi = mid;
            }
        }
        arr[lo..lo + k].to_vec()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let arr = vec![1, 2, 3, 4, 5];
        let k = 4;
        let x = 3;
        assert_eq!(Solution::find_closest_elements(arr, k, x), vec![1, 2, 3, 4]);

        let arr = vec![1, 2, 3, 4, 5];
        let k = 4;
        let x = -1;
        assert_eq!(Solution::find_closest_elements(arr, k, x), vec![1, 2, 3, 4]);
    }
}
