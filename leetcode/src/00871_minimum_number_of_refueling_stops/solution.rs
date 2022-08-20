use std::collections::BinaryHeap;

struct Solution;

impl Solution {
    pub fn min_refuel_stops(target: i32, start_fuel: i32, stations: Vec<Vec<i32>>) -> i32 {
        let mut pq = BinaryHeap::new();
        let mut cur_pos = start_fuel;
        let mut i = 0;
        let mut res = 0;
        while cur_pos < target {
            while i < stations.len() && stations[i][0] <= cur_pos {
                pq.push(stations[i][1]);
                i += 1;
            }
            if let Some(f) = pq.pop() {
                cur_pos += f;
                res += 1;
            } else {
                return -1;
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let target = 1;
        let start_fuel = 1;
        let stations = vec![];
        assert_eq!(Solution::min_refuel_stops(target, start_fuel, stations), 0);

        let target = 100;
        let start_fuel = 1;
        let stations = vec![vec![10, 100]];
        assert_eq!(Solution::min_refuel_stops(target, start_fuel, stations), -1);

        let target = 100;
        let start_fuel = 10;
        let stations = vec![vec![10, 60], vec![20, 30], vec![30, 30], vec![60, 40]];
        assert_eq!(Solution::min_refuel_stops(target, start_fuel, stations), 2);
    }
}
