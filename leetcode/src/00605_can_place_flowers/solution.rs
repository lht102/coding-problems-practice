struct Solution;

impl Solution {
    pub fn can_place_flowers(flowerbed: Vec<i32>, n: i32) -> bool {
        let mut arr = flowerbed;
        let mut k = n;
        for i in 0..arr.len() {
            let left = i >= 1 && arr[i - 1] == 1;
            let mid = arr[i] == 1;
            let right = i + 1 < arr.len() && arr[i + 1] == 1;
            if left || mid || right {
                continue;
            }
            arr[i] = 1;
            k -= 1;
        }
        k <= 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let flowerbed = vec![1, 0, 0, 0, 1];
        let n = 1;
        assert!(Solution::can_place_flowers(flowerbed, n));

        let flowerbed = vec![1, 0, 0, 0, 1];
        let n = 2;
        assert!(!Solution::can_place_flowers(flowerbed, n));
    }
}
