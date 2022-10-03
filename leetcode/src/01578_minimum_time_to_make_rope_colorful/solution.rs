struct Solution;

impl Solution {
    pub fn min_cost(colors: String, needed_time: Vec<i32>) -> i32 {
        let colors = colors.chars().collect::<Vec<_>>();
        let n = colors.len();
        let mut res = 0;
        let (mut i, mut j) = (0, 0);
        while j < n {
            let (mut cur_total, mut cur_max) = (0, 0);
            while j < n && colors[i] == colors[j] {
                cur_total += needed_time[j];
                cur_max = cur_max.max(needed_time[j]);
                j += 1;
            }
            res += cur_total - cur_max;
            i = j;
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let colors = String::from("abaac");
        let needed_time = vec![1, 2, 3, 4, 5];
        assert_eq!(Solution::min_cost(colors, needed_time), 3);

        let colors = String::from("abc");
        let needed_time = vec![1, 2, 3];
        assert_eq!(Solution::min_cost(colors, needed_time), 0);

        let colors = String::from("aabaa");
        let needed_time = vec![1, 2, 3, 4, 1];
        assert_eq!(Solution::min_cost(colors, needed_time), 2);
    }
}
