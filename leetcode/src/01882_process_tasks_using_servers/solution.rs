use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::iter::FromIterator;

struct Solution;

impl Solution {
    pub fn assign_tasks(servers: Vec<i32>, tasks: Vec<i32>) -> Vec<i32> {
        // weight, index
        let mut free_servers = BinaryHeap::from_iter(
            servers
                .iter()
                .enumerate()
                .map(|(i, &num)| (Reverse(num), Reverse(i))),
        );
        // end_time, index
        let mut used_servers: BinaryHeap<(Reverse<u64>, Reverse<usize>)> = BinaryHeap::new();
        let mut res = Vec::with_capacity(tasks.len());
        let mut current_t = 0u64;
        for (i, &task) in tasks.iter().enumerate() {
            if free_servers.is_empty() {
                if let Some(&(Reverse(end_t), _)) = used_servers.peek() {
                    if current_t < end_t {
                        current_t = end_t
                    }
                }
            }
            current_t = current_t.max(i as u64);

            while let Some(&(Reverse(end_t), Reverse(i))) = used_servers.peek() {
                if end_t > current_t {
                    break;
                }
                free_servers.push((Reverse(servers[i]), Reverse(i)));
                used_servers.pop();
            }

            if let Some((_, Reverse(i))) = free_servers.pop() {
                used_servers.push((Reverse(current_t + task as u64), Reverse(i)));
                res.push(i as i32);
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let servers = vec![3, 3, 2];
        let tasks = vec![1, 2, 3, 2, 1, 2];
        assert_eq!(
            Solution::assign_tasks(servers, tasks),
            vec![2, 2, 0, 2, 1, 2]
        );

        let servers = vec![5, 1, 4, 3, 2];
        let tasks = vec![2, 1, 2, 4, 5, 2, 1];
        assert_eq!(
            Solution::assign_tasks(servers, tasks),
            vec![1, 4, 1, 4, 1, 3, 2]
        );
    }
}
