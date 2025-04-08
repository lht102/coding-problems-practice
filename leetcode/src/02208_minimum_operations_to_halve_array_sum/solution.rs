use std::cmp::Ordering;
use std::collections::BinaryHeap;

struct Solution;

#[derive(PartialEq)]
struct NonNan(f64);

impl PartialOrd for NonNan {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for NonNan {}

impl Ord for NonNan {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Solution {
    pub fn halve_array(nums: Vec<i32>) -> i32 {
        let h_sum = nums.iter().map(|&n| n as f64).sum::<f64>() / 2.0;
        let mut max_heap: BinaryHeap<NonNan> = BinaryHeap::from(
            nums.into_iter()
                .map(|n| NonNan(n as f64))
                .collect::<Vec<NonNan>>(),
        );
        let mut res = 0;
        let mut c_sum = 0.0;
        while c_sum < h_sum {
            let v = max_heap.pop().unwrap().0 / 2.0;
            c_sum += v;
            max_heap.push(NonNan(v));
            res += 1;
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let nums = vec![5, 19, 8, 1];
        assert_eq!(Solution::halve_array(nums), 3);

        let nums = vec![3, 8, 20];
        assert_eq!(Solution::halve_array(nums), 3);
    }
}
