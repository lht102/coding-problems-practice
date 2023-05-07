struct Solution;

impl Solution {
    pub fn longest_obstacle_course_at_each_position(obstacles: Vec<i32>) -> Vec<i32> {
        let mut dp = vec![];
        obstacles
            .into_iter()
            .map(|num| {
                let i = dp.partition_point(|&x| x <= num);
                if i == dp.len() {
                    dp.push(num);
                } else {
                    dp[i] = num;
                }
                i as i32 + 1
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let obstacles = vec![1, 2, 3, 2];
        assert_eq!(
            Solution::longest_obstacle_course_at_each_position(obstacles),
            vec![1, 2, 3, 3]
        );

        let obstacles = vec![2, 2, 1];
        assert_eq!(
            Solution::longest_obstacle_course_at_each_position(obstacles),
            vec![1, 2, 1]
        );

        let obstacles = vec![3, 1, 5, 6, 4, 2];
        assert_eq!(
            Solution::longest_obstacle_course_at_each_position(obstacles),
            vec![1, 1, 2, 3, 2, 2]
        );
    }
}
