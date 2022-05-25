struct Solution;

impl Solution {
    pub fn max_envelopes(envelopes: Vec<Vec<i32>>) -> i32 {
        let mut envelopes = envelopes;
        envelopes.sort_by(|a, b| {
            if a[0] == b[0] {
                b[1].cmp(&a[1])
            } else {
                a[0].cmp(&b[0])
            }
        });
        let mut dp = vec![envelopes[0][1]];
        for e in envelopes.iter().skip(1) {
            let i = dp.partition_point(|&num| num < e[1]);
            if i == dp.len() {
                dp.push(e[1]);
            } else {
                dp[i] = e[1];
            }
        }
        dp.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let envelopes = vec![vec![5, 4], vec![6, 4], vec![6, 7], vec![2, 3]];
        assert_eq!(Solution::max_envelopes(envelopes), 3);

        let envelopes = vec![vec![1, 1], vec![1, 1], vec![1, 1]];
        assert_eq!(Solution::max_envelopes(envelopes), 1);
    }
}
