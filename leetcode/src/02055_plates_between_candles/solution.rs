struct Solution;

impl Solution {
    pub fn plates_between_candles(s: String, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let candle_indices = s
            .chars()
            .enumerate()
            .filter_map(|(i, ch)| (ch == '|').then(|| i))
            .collect::<Vec<_>>();
        queries
            .into_iter()
            .map(|q| {
                let left_i = candle_indices.partition_point(|&i| i < q[0] as usize);
                if left_i == candle_indices.len() {
                    return 0;
                }
                let right_i = candle_indices.partition_point(|&i| i <= q[1] as usize);
                if right_i < 1 || left_i >= right_i - 1 {
                    return 0;
                }
                (candle_indices[right_i - 1] + 1 - candle_indices[left_i] - (right_i - left_i))
                    as i32
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let s = String::from("**|**|***|");
        let queries = vec![vec![2, 5], vec![5, 9]];
        assert_eq!(Solution::plates_between_candles(s, queries), vec![2, 3]);

        let s = String::from("***|**|*****|**||**|*");
        let queries = vec![
            vec![1, 17],
            vec![4, 5],
            vec![14, 17],
            vec![5, 11],
            vec![15, 16],
        ];
        assert_eq!(
            Solution::plates_between_candles(s, queries),
            vec![9, 0, 0, 0, 0]
        );
    }
}
