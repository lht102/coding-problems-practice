use std::collections::BinaryHeap;

struct Solution;

impl Solution {
    pub fn minimum_deviation(nums: Vec<i32>) -> i32 {
        let transformed_nums = nums
            .iter()
            .map(|&num| if num % 2 == 1 { num * 2 } else { num })
            .collect::<Vec<_>>();
        let mut min_num = *transformed_nums.iter().min().unwrap();
        let mut pq = BinaryHeap::from(transformed_nums);
        let mut res = i32::MAX;
        while let Some(max_num) = pq.pop() {
            res = res.min(max_num - min_num);
            if max_num % 2 == 1 {
                break;
            }
            let smaller_num = max_num / 2;
            min_num = min_num.min(smaller_num);
            pq.push(smaller_num);
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let nums = vec![1, 2, 3, 4];
        assert_eq!(Solution::minimum_deviation(nums), 1);

        let nums = vec![4, 1, 5, 20, 3];
        assert_eq!(Solution::minimum_deviation(nums), 3);
    }
}
