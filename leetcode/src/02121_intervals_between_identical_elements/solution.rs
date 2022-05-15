use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn get_distances(arr: Vec<i32>) -> Vec<i64> {
        let num_to_indices =
            arr.iter()
                .enumerate()
                .fold(HashMap::<i32, Vec<usize>>::new(), |mut map, (i, &v)| {
                    map.entry(v).or_default().push(i);
                    map
                });
        let mut res = vec![0; arr.len()];
        for indices in num_to_indices.values() {
            let psum = std::iter::once(0)
                .chain(indices.iter().scan(0, |sum, i| {
                    *sum += i;
                    Some(*sum)
                }))
                .collect::<Vec<_>>();
            for (i, &idx) in indices.iter().enumerate() {
                res[idx] = ((idx * i - psum[i])
                    + (psum[indices.len()] - psum[i] - idx * (indices.len() - i)))
                    as i64
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
        let arr = vec![2, 1, 3, 1, 2, 3, 3];
        assert_eq!(Solution::get_distances(arr), vec![4, 2, 7, 2, 4, 4, 5]);

        let arr = vec![10, 5, 10, 10];
        assert_eq!(Solution::get_distances(arr), vec![5, 0, 3, 4]);
    }
}
