struct Solution;

pub unsafe fn guess(num: i32) -> i32 {
    num
}

impl Solution {
    #[allow(non_snake_case)]
    unsafe fn guessNumber(n: i32) -> i32 {
        let (mut lo, mut hi) = (1, n);
        while lo < hi {
            let mid = lo + (hi - lo) / 2;
            if guess(mid) == 1 {
                lo = mid + 1;
            } else {
                hi = mid;
            }
        }
        lo
    }
}
