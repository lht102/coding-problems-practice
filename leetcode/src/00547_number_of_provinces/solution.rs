struct Solution;

impl Solution {
    pub fn find_circle_num(is_connected: Vec<Vec<i32>>) -> i32 {
        let n = is_connected.len();
        let g = is_connected
            .into_iter()
            .map(|row| row.into_iter().map(|x| x as usize).collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let mut visited = vec![false; n];
        (0..n)
            .filter_map(|i| {
                if !visited[i] {
                    Self::dfs(&g, i, &mut visited);
                    Some(())
                } else {
                    None
                }
            })
            .count() as _
    }

    fn dfs(g: &[Vec<usize>], i: usize, visited: &mut [bool]) {
        if visited[i] {
            return;
        }
        visited[i] = true;
        for (j, _) in g[i]
            .iter()
            .enumerate()
            .filter(|&(j, &connected)| i != j && connected == 1)
        {
            Self::dfs(g, j, visited);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let is_connected = vec![vec![1, 1, 0], vec![1, 1, 0], vec![0, 0, 1]];
        assert_eq!(Solution::find_circle_num(is_connected), 2);

        let is_connected = vec![vec![1, 0, 0], vec![0, 1, 0], vec![0, 0, 1]];
        assert_eq!(Solution::find_circle_num(is_connected), 3);
    }
}
