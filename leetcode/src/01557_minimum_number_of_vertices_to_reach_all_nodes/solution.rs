struct Solution;

impl Solution {
    pub fn find_smallest_set_of_vertices(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        let indegrees = edges.iter().fold(vec![0; n as usize], |mut acc, e| {
            acc[e[1] as usize] += 1;
            acc
        });
        indegrees
            .iter()
            .enumerate()
            .filter_map(|(i, &indegree)| (indegree == 0).then_some(i as i32))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let n = 6;
        let edges = vec![vec![0, 1], vec![0, 2], vec![2, 5], vec![3, 4], vec![4, 2]];
        assert_eq!(
            Solution::find_smallest_set_of_vertices(n, edges),
            vec![0, 3],
        );

        let n = 5;
        let edges = vec![vec![0, 1], vec![2, 1], vec![3, 1], vec![1, 4], vec![2, 4]];
        assert_eq!(
            Solution::find_smallest_set_of_vertices(n, edges),
            vec![0, 2, 3],
        );
    }
}
