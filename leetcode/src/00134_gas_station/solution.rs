struct Solution;

impl Solution {
    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        let n = gas.len();
        let mut start = 0;
        let mut tank = 0;
        for i in 0..n {
            tank += gas[i] - cost[i];
            if tank < 0 {
                start = i + 1;
                tank = 0;
            }
        }
        if gas.iter().sum::<i32>() - cost.iter().sum::<i32>() < 0 {
            -1
        } else {
            start as i32
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let gas = vec![1, 2, 3, 4, 5];
        let cost = vec![3, 4, 5, 1, 2];
        assert_eq!(Solution::can_complete_circuit(gas, cost), 3);

        let gas = vec![2, 3, 4];
        let cost = vec![3, 4, 3];
        assert_eq!(Solution::can_complete_circuit(gas, cost), -1);
    }
}
