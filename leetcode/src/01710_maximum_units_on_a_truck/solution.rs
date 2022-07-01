struct Solution;

impl Solution {
    pub fn maximum_units(box_types: Vec<Vec<i32>>, truck_size: i32) -> i32 {
        let mut box_types = box_types;
        box_types.sort_unstable_by(|a, b| b[1].cmp(&a[1]));
        let mut res = 0;
        let mut sz = truck_size;
        for b in &box_types {
            let taken = sz.min(b[0]);
            res += taken * b[1];
            sz -= taken;
            if taken == 0 {
                break;
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
        let box_types = vec![vec![1, 3], vec![2, 2], vec![3, 1]];
        let truck_size = 4;
        assert_eq!(Solution::maximum_units(box_types, truck_size), 8);

        let box_types = vec![vec![5, 10], vec![2, 5], vec![4, 7], vec![3, 9]];
        let truck_size = 10;
        assert_eq!(Solution::maximum_units(box_types, truck_size), 91);
    }
}
