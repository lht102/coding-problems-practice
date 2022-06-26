use std::collections::HashMap;
use std::iter::FromIterator;

struct Solution;

struct UnionFind {
    p: Vec<usize>,
    sz: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        Self {
            p: Vec::from_iter(0..n),
            sz: vec![1; n],
        }
    }

    fn find(&mut self, v: usize) -> usize {
        if v == self.p[v] {
            return v;
        }
        self.p[v] = self.find(self.p[v]);
        self.p[v]
    }

    fn union(&mut self, a: usize, b: usize) {
        let u = self.find(a);
        let v = self.find(b);
        if u == v {
            return;
        }
        if self.sz[u] < self.sz[v] {
            self.p[u] = v;
            self.sz[v] += self.sz[u];
        } else {
            self.p[v] = u;
            self.sz[u] += self.sz[v];
        }
    }
}

impl Solution {
    pub fn count_pairs(n: i32, edges: Vec<Vec<i32>>) -> i64 {
        let mut num_of_groups = n as usize;
        let mut uf = UnionFind::new(num_of_groups);
        for e in &edges {
            uf.union(e[0] as usize, e[1] as usize);
        }
        let mut groups = HashMap::new();
        for i in 0..num_of_groups {
            *groups.entry(uf.find(i)).or_default() += 1;
        }
        let mut res = 0;
        for sz in groups.values() {
            num_of_groups -= sz;
            res += (sz * num_of_groups) as i64;
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let n = 3;
        let edges = vec![vec![0, 1], vec![0, 2], vec![1, 2]];
        assert_eq!(Solution::count_pairs(n, edges), 0);

        let n = 7;
        let edges = vec![vec![0, 2], vec![0, 5], vec![2, 4], vec![1, 6], vec![5, 4]];
        assert_eq!(Solution::count_pairs(n, edges), 14);

        let n = 11;
        let edges = vec![
            vec![5, 0],
            vec![1, 0],
            vec![10, 7],
            vec![9, 8],
            vec![7, 2],
            vec![1, 3],
            vec![0, 2],
            vec![8, 5],
            vec![4, 6],
            vec![4, 2],
        ];
        assert_eq!(Solution::count_pairs(n, edges), 0);
    }
}
