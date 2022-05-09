use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn find_original_array(changed: Vec<i32>) -> Vec<i32> {
        let mut freq = changed
            .iter()
            .fold(HashMap::<i32, usize>::new(), |mut map, &num| {
                *map.entry(num).or_default() += 1;
                map
            });
        let mut changed = changed;
        changed.sort_unstable();
        let mut res = Vec::new();
        for &num in &changed {
            if *freq.get(&num).unwrap() > 0 {
                match freq.get_mut(&(num * 2)) {
                    Some(cnt) => {
                        if num == num * 2 {
                            if *cnt < 2 {
                                return vec![];
                            }
                            *cnt -= 2;
                        } else {
                            if *cnt < 1 {
                                return vec![];
                            }
                            *cnt -= 1;
                            *freq.entry(num).or_default() -= 1;
                        }
                        res.push(num);
                    }
                    None => return vec![],
                }
            }
        }
        if res.len() * 2 == changed.len() {
            res
        } else {
            vec![]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let changed = vec![1, 3, 4, 2, 6, 8];
        assert_eq!(Solution::find_original_array(changed), vec![1, 3, 4]);

        let changed = vec![6, 3, 0, 1];
        assert_eq!(Solution::find_original_array(changed), vec![]);

        let changed = vec![1];
        assert_eq!(Solution::find_original_array(changed), vec![]);
    }
}
