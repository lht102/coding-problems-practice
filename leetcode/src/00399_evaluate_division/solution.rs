use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn calc_equation(
        equations: Vec<Vec<String>>,
        values: Vec<f64>,
        queries: Vec<Vec<String>>,
    ) -> Vec<f64> {
        let n = equations.len();
        let mut num_v = 0;
        let mut s_to_v = HashMap::<String, usize>::new();
        for i in 0..n {
            let from = equations[i][0].clone();
            if !s_to_v.contains_key(&from) {
                s_to_v.insert(from, num_v);
                num_v += 1;
            }
            let to = equations[i][1].clone();
            if !s_to_v.contains_key(&to) {
                s_to_v.insert(to, num_v);
                num_v += 1;
            }
        }
        let mut dp = vec![vec![-1.0; num_v]; num_v];
        for i in 0..n {
            let from = *s_to_v.get(&equations[i][0]).unwrap();
            let to = *s_to_v.get(&equations[i][1]).unwrap();
            dp[from][to] = values[i];
            dp[to][from] = 1.0 / values[i];
            dp[from][from] = 1.0;
            dp[to][to] = 1.0;
        }
        for k in 0..num_v {
            for i in 0..num_v {
                for j in 0..num_v {
                    if dp[i][k] != -1.0 && dp[k][j] != -1.0 {
                        dp[i][j] = dp[i][k] * dp[k][j];
                    }
                }
            }
        }
        queries
            .iter()
            .map(|q| {
                let from = s_to_v.get(&q[0]);
                let to = s_to_v.get(&q[1]);
                if from.is_none() || to.is_none() {
                    return -1.0;
                }
                (dp[*from.unwrap()][*to.unwrap()] * 100000.0).round() / 100000.0
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let equations = vec![
            vec![String::from("a"), String::from("b")],
            vec![String::from("b"), String::from("c")],
        ];
        let values = vec![2.0, 3.0];
        let queries = vec![
            vec![String::from("a"), String::from("c")],
            vec![String::from("b"), String::from("a")],
            vec![String::from("a"), String::from("e")],
            vec![String::from("a"), String::from("a")],
            vec![String::from("x"), String::from("x")],
        ];
        assert_eq!(
            Solution::calc_equation(equations, values, queries),
            vec![6.0, 0.5, -1.0, 1.0, -1.0]
        );

        let equations = vec![
            vec![String::from("a"), String::from("b")],
            vec![String::from("b"), String::from("c")],
            vec![String::from("bc"), String::from("cd")],
        ];
        let values = vec![1.5, 2.5, 5.0];
        let queries = vec![
            vec![String::from("a"), String::from("c")],
            vec![String::from("c"), String::from("b")],
            vec![String::from("bc"), String::from("cd")],
            vec![String::from("cd"), String::from("bc")],
        ];
        assert_eq!(
            Solution::calc_equation(equations, values, queries),
            vec![3.75, 0.4, 5.0, 0.2]
        );

        let equations = vec![vec![String::from("a"), String::from("b")]];
        let values = vec![0.5];
        let queries = vec![
            vec![String::from("a"), String::from("b")],
            vec![String::from("b"), String::from("a")],
            vec![String::from("a"), String::from("c")],
            vec![String::from("x"), String::from("y")],
        ];
        assert_eq!(
            Solution::calc_equation(equations, values, queries),
            vec![0.5, 2.0, -1.0, -1.0]
        );
    }
}
