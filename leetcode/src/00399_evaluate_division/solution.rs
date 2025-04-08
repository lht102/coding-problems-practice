use std::collections::HashMap;
use std::collections::hash_map::Entry;

struct Solution;

impl Solution {
    pub fn calc_equation(
        equations: Vec<Vec<String>>,
        values: Vec<f64>,
        queries: Vec<Vec<String>>,
    ) -> Vec<f64> {
        let mut num_of_nodes = 0;
        let mut str_to_node_idx = HashMap::<&str, usize>::new();
        for equation in &equations {
            let from = &equation[0];
            if let Entry::Vacant(e) = str_to_node_idx.entry(from) {
                e.insert(num_of_nodes);
                num_of_nodes += 1;
            }
            let to = &equation[1];
            if let Entry::Vacant(e) = str_to_node_idx.entry(to) {
                e.insert(num_of_nodes);
                num_of_nodes += 1;
            }
        }
        let mut dp = vec![vec![-1.0; num_of_nodes]; num_of_nodes];
        for (equation, &value) in equations.iter().zip(values.iter()) {
            let from = *str_to_node_idx.get(&equation[0].as_ref()).unwrap();
            let to = *str_to_node_idx.get(&equation[1].as_ref()).unwrap();
            dp[from][to] = value;
            dp[to][from] = 1.0 / value;
            dp[from][from] = 1.0;
            dp[to][to] = 1.0;
        }
        for k in 0..num_of_nodes {
            for i in 0..num_of_nodes {
                for j in 0..num_of_nodes {
                    if dp[i][k] != -1.0 && dp[k][j] != -1.0 {
                        dp[i][j] = dp[i][k] * dp[k][j];
                    }
                }
            }
        }
        queries
            .into_iter()
            .map(|q| {
                let from = str_to_node_idx.get(&q[0].as_ref());
                let to = str_to_node_idx.get(&q[1].as_ref());
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
