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
    pub fn distance_limited_paths_exist(
        n: i32,
        edge_list: Vec<Vec<i32>>,
        queries: Vec<Vec<i32>>,
    ) -> Vec<bool> {
        let mut edge_list = edge_list;
        let mut queries = queries.into_iter().enumerate().collect::<Vec<_>>();
        edge_list.sort_unstable_by(|a, b| a[2].cmp(&b[2]));
        queries.sort_unstable_by(|a, b| a.1.get(2).cmp(&b.1.get(2)));
        let mut uf = UnionFind::new(n as usize);
        let mut res = vec![false; queries.len()];
        let mut j = 0;
        for (i, q) in queries {
            while j < edge_list.len() && edge_list[j][2] < q[2] {
                uf.union(edge_list[j][0] as usize, edge_list[j][1] as usize);
                j += 1;
            }
            res[i] = uf.find(q[0] as usize) == uf.find(q[1] as usize)
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
        let edge_list = vec![vec![0, 1, 2], vec![1, 2, 4], vec![2, 0, 8], vec![1, 0, 16]];
        let queries = vec![vec![0, 1, 2], vec![0, 2, 5]];
        assert_eq!(
            Solution::distance_limited_paths_exist(n, edge_list, queries),
            vec![false, true]
        );

        let n = 5;
        let edge_list = vec![vec![0, 1, 10], vec![1, 2, 5], vec![2, 3, 9], vec![3, 4, 13]];
        let queries = vec![vec![0, 4, 14], vec![1, 4, 13]];
        assert_eq!(
            Solution::distance_limited_paths_exist(n, edge_list, queries),
            vec![true, false]
        );
    }
}
