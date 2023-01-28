struct Solution;

impl Solution {
    pub fn max_ice_cream(costs: Vec<i32>, coins: i32) -> i32 {
        let mut costs = costs;
        Solution::counting_sort(&mut costs, |&n| n as usize);
        let mut res = 0;
        let mut coins = coins;
        while res < costs.len() && costs[res] <= coins {
            coins -= costs[res];
            res += 1;
        }
        res as _
    }

    fn counting_sort<F, T>(arr: &mut [T], key: F)
    where
        F: Fn(&T) -> usize,
        T: Clone,
    {
        if arr.is_empty() {
            return;
        }
        let vals = arr.iter().map(&key).collect::<Vec<_>>();
        let min = *vals.iter().min().unwrap();
        let max = *vals.iter().max().unwrap();
        let mut psum = arr
            .iter()
            .fold(vec![0; max + 1 - min], |mut cnts, val| {
                cnts[key(val) - min] += 1;
                cnts
            })
            .into_iter()
            .scan(0, |acc, cnt| {
                *acc += cnt;
                Some(*acc - cnt)
            })
            .collect::<Vec<_>>();
        for val in arr.to_vec().iter() {
            let idx = key(val) - min;
            arr[psum[idx]] = val.clone();
            psum[idx] += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let costs = vec![1, 3, 2, 4, 1];
        let coins = 7;
        assert_eq!(Solution::max_ice_cream(costs, coins), 4);

        // let costs = vec![10, 6, 8, 7, 7, 8];
        // let coins = 5;
        // assert_eq!(Solution::max_ice_cream(costs, coins), 0);
        //
        // let costs = vec![1, 6, 3, 1, 2, 5];
        // let coins = 20;
        // assert_eq!(Solution::max_ice_cream(costs, coins), 6);
    }
}
