use std::collections::HashMap;
use std::collections::VecDeque;

struct Solution;

impl Solution {
    pub fn min_jumps(arr: Vec<i32>) -> i32 {
        let mut g =
            arr.iter()
                .enumerate()
                .fold(HashMap::<i32, Vec<i32>>::new(), |mut map, (i, &num)| {
                    map.entry(num).or_default().push(i as i32);
                    map
                });
        let n = arr.len();
        let mut q = VecDeque::from([0]);
        let mut visited = vec![false; n];
        visited[0] = true;
        let mut res = 0;
        while !q.is_empty() {
            for _ in 0..q.len() {
                if let Some(i) = q.pop_front() {
                    if i == n as i32 - 1 {
                        return res;
                    }
                    if let Some(next) = g.get_mut(&arr[i as usize]) {
                        next.push(i + 1);
                        next.push(i - 1);
                        for &j in next.iter() {
                            if j >= 0 && j < n as i32 && !visited[j as usize] {
                                visited[j as usize] = true;
                                q.push_back(j);
                            }
                        }
                        next.clear();
                    }
                }
            }
            res += 1;
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let arr = vec![100, -23, -23, 404, 100, 23, 23, 23, 3, 404];
        assert_eq!(Solution::min_jumps(arr), 3);

        let arr = vec![7];
        assert_eq!(Solution::min_jumps(arr), 0);

        let arr = vec![7, 6, 9, 6, 9, 6, 9, 7];
        assert_eq!(Solution::min_jumps(arr), 1);
    }
}
