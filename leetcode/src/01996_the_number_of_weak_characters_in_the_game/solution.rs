struct Solution;

impl Solution {
    pub fn number_of_weak_characters(properties: Vec<Vec<i32>>) -> i32 {
        let mut properties = properties;
        properties.sort_unstable_by(|a, b| {
            if a[0] == b[0] {
                b[1].cmp(&a[1])
            } else {
                a[0].cmp(&b[0])
            }
        });
        let mut res = 0;
        let mut def = -1;
        for arr in properties.iter().rev() {
            if arr[1] < def {
                res += 1;
            }
            def = def.max(arr[1]);
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let properties = vec![vec![5, 5], vec![6, 3], vec![3, 6]];
        assert_eq!(Solution::number_of_weak_characters(properties), 0);

        let properties = vec![vec![2, 2], vec![3, 3]];
        assert_eq!(Solution::number_of_weak_characters(properties), 1);

        let properties = vec![vec![1, 5], vec![10, 4], vec![4, 3]];
        assert_eq!(Solution::number_of_weak_characters(properties), 1);
    }
}
