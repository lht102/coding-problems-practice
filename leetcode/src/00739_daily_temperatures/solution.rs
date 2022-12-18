struct Solution;

impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let mut res = vec![0; temperatures.len()];
        let mut st = Vec::new();
        for (i, &t) in temperatures.iter().enumerate() {
            while !st.is_empty() && temperatures[*st.last().unwrap()] < t {
                let j = st.pop().unwrap();
                res[j] = (i - j) as i32;
            }
            st.push(i);
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let temperatures = vec![73, 74, 75, 71, 69, 72, 76, 73];
        assert_eq!(
            Solution::daily_temperatures(temperatures),
            vec![1, 1, 4, 2, 1, 1, 0, 0]
        );

        let temperatures = vec![30, 40, 50, 60];
        assert_eq!(Solution::daily_temperatures(temperatures), vec![1, 1, 1, 0]);

        let temperatures = vec![30, 60, 90];
        assert_eq!(Solution::daily_temperatures(temperatures), vec![1, 1, 0]);
    }
}
