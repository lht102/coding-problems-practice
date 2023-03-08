struct Solution;

impl Solution {
    pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
        let (mut lo, mut hi) = (1, *piles.iter().max().unwrap());
        while lo < hi {
            let mid = (hi - lo) / 2 + lo;
            if Self::is_valid(&piles, h, mid) {
                hi = mid;
            } else {
                lo = mid + 1;
            }
        }
        lo
    }

    fn is_valid(piles: &[i32], h: i32, k: i32) -> bool {
        let mut required_h = 0;
        for pile in piles {
            required_h += (pile + k - 1) / k;
            if required_h > h {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let piles = vec![3, 6, 7, 11];
        let h = 8;
        assert_eq!(Solution::min_eating_speed(piles, h), 4);

        let piles = vec![30, 11, 23, 4, 20];
        let h = 5;
        assert_eq!(Solution::min_eating_speed(piles, h), 30);

        let piles = vec![30, 11, 23, 4, 20];
        let h = 6;
        assert_eq!(Solution::min_eating_speed(piles, h), 23);

        let piles = vec![
            332484035, 524908576, 855865114, 632922376, 222257295, 690155293, 112677673, 679580077,
            337406589, 290818316, 877337160, 901728858, 679284947, 688210097, 692137887, 718203285,
            629455728, 941802184,
        ];
        let h = 823855818;
        assert_eq!(Solution::min_eating_speed(piles, h), 14);
    }
}
