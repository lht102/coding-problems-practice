use std::collections::VecDeque;

struct Solution;

impl Solution {
    pub fn minimum_operations(nums: Vec<i32>, start: i32, goal: i32) -> i32 {
        let functions: &[fn(i32, i32) -> i32] = &[Self::add, Self::sub, Self::xor];
        let mut vi = vec![false; 1001];
        let mut q = VecDeque::new();
        q.push_back((0, start));
        vi[start as usize] = true;
        while let Some((steps, x)) = q.pop_front() {
            for &num in &nums {
                for f in functions {
                    let new_x = f(x, num);
                    if new_x == goal {
                        return steps + 1;
                    }
                    if !(0..=1000).contains(&new_x) {
                        continue;
                    }
                    if !vi[new_x as usize] {
                        vi[new_x as usize] = true;
                        q.push_back((steps + 1, new_x));
                    }
                }
            }
        }
        -1
    }

    fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    fn sub(a: i32, b: i32) -> i32 {
        a - b
    }

    fn xor(a: i32, b: i32) -> i32 {
        a ^ b
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let nums = vec![2, 4, 12];
        let start = 2;
        let goal = 12;
        assert_eq!(Solution::minimum_operations(nums, start, goal), 2);

        let nums = vec![3, 5, 7];
        let start = 0;
        let goal = -4;
        assert_eq!(Solution::minimum_operations(nums, start, goal), 2);

        let nums = vec![2, 8, 16];
        let start = 0;
        let goal = 1;
        assert_eq!(Solution::minimum_operations(nums, start, goal), -1);
    }
}
