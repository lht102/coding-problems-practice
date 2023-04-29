struct Solution;

impl Solution {
    pub fn num_similar_groups(strs: Vec<String>) -> i32 {
        let n = strs.len();
        let mut adj = vec![vec![]; n];
        for i in 0..n {
            for j in 0..n {
                if Self::is_similar(&strs[i], &strs[j]) {
                    adj[i].push(j);
                    adj[j].push(i);
                }
            }
        }

        let mut visited = vec![false; n];
        (0..n).fold(0, |res, i| {
            if !visited[i] {
                Self::dfs(&adj, &mut visited, i);
                res + 1
            } else {
                res
            }
        })
    }

    fn dfs(adj: &[Vec<usize>], visited: &mut [bool], source: usize) {
        visited[source] = true;
        for &destination in &adj[source] {
            if !visited[destination] {
                Self::dfs(adj, visited, destination);
            }
        }
    }

    fn is_similar(a: &str, b: &str) -> bool {
        let diff_cnt = a.chars().zip(b.chars()).filter(|(c1, c2)| c1 != c2).count();
        diff_cnt == 0 || diff_cnt == 2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let strs = vec![
            String::from("tars"),
            String::from("rats"),
            String::from("arts"),
            String::from("star"),
        ];
        assert_eq!(Solution::num_similar_groups(strs), 2);

        let strs = vec![String::from("omv"), String::from("ovm")];
        assert_eq!(Solution::num_similar_groups(strs), 1);
    }
}
