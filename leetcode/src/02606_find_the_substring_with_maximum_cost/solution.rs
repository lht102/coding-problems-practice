struct Solution;

impl Solution {
    pub fn maximum_cost_substring(s: String, chars: String, vals: Vec<i32>) -> i32 {
        let get_index = |x: u8| (x - b'a') as usize;
        let ch_costs = chars.bytes().zip(vals.iter()).fold(
            (1..=26).collect::<Vec<_>>(),
            |mut ch_costs, (ch, &val)| {
                ch_costs[get_index(ch)] = val;
                ch_costs
            },
        );
        let mut res = 0;
        let mut cur_sum = 0;
        for val in s.bytes().map(|ch| ch_costs[get_index(ch)]) {
            cur_sum = 0.max(cur_sum + val);
            res = res.max(cur_sum);
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let s = String::from("adaa");
        let chars = String::from("d");
        let vals = vec![-1000];
        assert_eq!(Solution::maximum_cost_substring(s, chars, vals), 2);

        let s = String::from("abc");
        let chars = String::from("abc");
        let vals = vec![-1, -1, -1];
        assert_eq!(Solution::maximum_cost_substring(s, chars, vals), 0);
    }
}
