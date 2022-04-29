use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn minimum_operations(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n < 2 {
            return 0;
        }
        let even_freq =
            nums.iter()
                .step_by(2)
                .fold(HashMap::<i32, usize>::new(), |mut map, &num| {
                    *map.entry(num).or_default() += 1;
                    map
                });
        let odd_freq =
            nums.iter()
                .skip(1)
                .step_by(2)
                .fold(HashMap::<i32, usize>::new(), |mut map, &num| {
                    *map.entry(num).or_default() += 1;
                    map
                });
        let max_even = even_freq.iter().map(|(&k, &v)| (v, k)).max().unwrap();
        let max_odd = odd_freq.iter().map(|(&k, &v)| (v, k)).max().unwrap();
        if max_even.1 != max_odd.1 {
            return (n - max_even.0 - max_odd.0) as i32;
        }
        let find_second_max_cnt_fn = |map: &HashMap<i32, usize>, max_cnt_ch: i32| -> usize {
            map.iter().fold(0, |second_max, (&k, &v)| {
                if k != max_cnt_ch && v > second_max {
                    return v;
                }
                second_max
            })
        };
        let second_max_even_cnt = find_second_max_cnt_fn(&even_freq, max_even.1);
        let second_max_odd_cnt = find_second_max_cnt_fn(&odd_freq, max_odd.1);
        (n - max_even.0 - second_max_odd_cnt).min(n - max_odd.0 - second_max_even_cnt) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let nums = vec![3, 1, 3, 2, 4, 3];
        assert_eq!(Solution::minimum_operations(nums), 3);

        let nums = vec![1, 2, 2, 2, 2];
        assert_eq!(Solution::minimum_operations(nums), 2);

        let nums = vec![
            35, 67, 46, 81, 34, 66, 52, 70, 19, 71, 23, 34, 2, 30, 98, 55, 69, 40, 3, 95, 12, 64,
            56, 32, 45, 71, 82, 72, 53, 90, 43, 30, 71, 42, 26, 10, 93, 24, 30, 42, 70, 15, 69, 13,
            53, 85, 12, 17, 21, 41, 5, 26, 32, 53, 95, 28, 97, 87, 87, 50, 15, 32, 72, 29, 29, 80,
            41, 96, 47, 39, 31, 82, 61, 42, 12, 5, 9, 53, 92, 57, 11, 82, 82, 4, 66, 99, 15, 81,
            72, 99, 47,
        ];
        assert_eq!(Solution::minimum_operations(nums), 85);
    }
}
