struct Solution;

impl Solution {
    pub fn maximum_requests(n: i32, requests: Vec<Vec<i32>>) -> i32 {
        Self::solve(&requests, 0, &mut vec![0; n as usize])
    }

    fn solve(requests: &[Vec<i32>], i: usize, indegrees: &mut [i32]) -> i32 {
        if i >= requests.len() {
            return if indegrees.iter().filter(|&&x| x == 0).count() == indegrees.len() {
                0
            } else {
                i32::MIN
            };
        }
        let (u, v) = (requests[i][0] as usize, requests[i][1] as usize);
        indegrees[u] -= 1;
        indegrees[v] += 1;
        let take = 1 + Self::solve(requests, i + 1, indegrees);
        indegrees[v] -= 1;
        indegrees[u] += 1;
        take.max(Self::solve(requests, i + 1, indegrees))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let n = 5;
        let requests = vec![
            vec![0, 1],
            vec![1, 0],
            vec![0, 1],
            vec![1, 2],
            vec![2, 0],
            vec![3, 4],
        ];
        assert_eq!(Solution::maximum_requests(n, requests), 5);

        let n = 3;
        let requests = vec![vec![0, 0], vec![1, 2], vec![2, 1]];
        assert_eq!(Solution::maximum_requests(n, requests), 3);

        let n = 4;
        let requests = vec![vec![0, 3], vec![3, 1], vec![1, 2], vec![2, 0]];
        assert_eq!(Solution::maximum_requests(n, requests), 4);
    }
}
