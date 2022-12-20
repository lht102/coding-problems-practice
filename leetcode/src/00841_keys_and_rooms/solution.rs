use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn can_visit_all_rooms(rooms: Vec<Vec<i32>>) -> bool {
        let mut adj = vec![vec![]; rooms.len()];
        for (i, room) in rooms.iter().enumerate() {
            for &key in room {
                adj[i].push(key as usize);
            }
        }
        Solution::dfs(&adj, &mut HashSet::new(), 0)
    }

    fn dfs(adj: &Vec<Vec<usize>>, visited: &mut HashSet<usize>, cur: usize) -> bool {
        if visited.contains(&cur) {
            return false;
        }
        visited.insert(cur);
        if visited.len() == adj.len() {
            return true;
        }
        for &next in &adj[cur] {
            if Solution::dfs(adj, visited, next) {
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let rooms = vec![vec![1], vec![2], vec![3], vec![]];
        assert!(Solution::can_visit_all_rooms(rooms));

        let rooms = vec![vec![1, 3], vec![3, 0, 1], vec![2], vec![0]];
        assert!(!Solution::can_visit_all_rooms(rooms));
    }
}
