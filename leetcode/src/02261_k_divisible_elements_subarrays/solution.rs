use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn count_distinct(nums: Vec<i32>, k: i32, p: i32) -> i32 {
        let n = nums.len();
        let mut set: HashSet<String> = HashSet::new();
        for i in 0..n {
            let mut cur_arr = String::new();
            let mut cur_k = 0;
            for num in nums.iter().take(n).skip(i) {
                if num % p == 0 {
                    cur_k += 1;
                }
                if cur_k > k {
                    break;
                }
                cur_arr.push_str(&num.to_string());
                cur_arr.push(' ');
                set.insert(cur_arr.clone());
            }
        }
        set.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let nums = vec![2, 3, 3, 2, 2];
        let k = 2;
        let p = 2;
        assert_eq!(Solution::count_distinct(nums, k, p), 11);

        let nums = vec![1, 2, 3, 4];
        let k = 4;
        let p = 1;
        assert_eq!(Solution::count_distinct(nums, k, p), 10);
    }
}
