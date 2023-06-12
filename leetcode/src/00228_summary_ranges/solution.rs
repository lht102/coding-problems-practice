struct Solution;

impl Solution {
    pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
        let mut i = 0;
        let mut res = vec![];
        while i < nums.len() {
            let mut j = i;
            while j + 1 < nums.len() && nums[j] + 1 == nums[j + 1] {
                j += 1;
            }
            if i != j {
                res.push(format!("{}->{}", nums[i], nums[j]));
                i = j + 1;
            } else {
                res.push(nums[i].to_string());
                i += 1;
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
        let nums = vec![0, 1, 2, 4, 5, 7];
        assert_eq!(
            Solution::summary_ranges(nums),
            vec![
                String::from("0->2"),
                String::from("4->5"),
                String::from("7")
            ]
        );

        let nums = vec![0, 2, 3, 4, 6, 8, 9];
        assert_eq!(
            Solution::summary_ranges(nums),
            vec![
                String::from("0"),
                String::from("2->4"),
                String::from("6"),
                String::from("8->9")
            ]
        );
    }
}
