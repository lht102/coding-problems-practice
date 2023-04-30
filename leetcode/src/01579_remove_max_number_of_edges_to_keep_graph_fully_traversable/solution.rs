use std::iter::FromIterator;

struct Solution;

struct UnionFind {
    p: Vec<usize>,
    sz: Vec<usize>,
    num_of_connected_components: usize,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        Self {
            p: Vec::from_iter(0..n),
            sz: vec![1; n],
            num_of_connected_components: n,
        }
    }

    fn find(&mut self, v: usize) -> usize {
        if v == self.p[v] {
            return v;
        }
        self.p[v] = self.find(self.p[v]);
        self.p[v]
    }

    fn union(&mut self, a: usize, b: usize) -> bool {
        let u = self.find(a);
        let v = self.find(b);
        if u == v {
            return false;
        }
        if self.sz[u] < self.sz[v] {
            self.p[u] = v;
            self.sz[v] += self.sz[u];
        } else {
            self.p[v] = u;
            self.sz[u] += self.sz[v];
        }
        self.num_of_connected_components -= 1;
        true
    }
}

impl Solution {
    pub fn max_num_edges_to_remove(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let edges = edges
            .into_iter()
            .map(|e| [e[0] as usize, (e[1] - 1) as usize, (e[2] - 1) as usize])
            .collect::<Vec<_>>();
        let mut ufa = UnionFind::new(n);
        let mut ufb = UnionFind::new(n);
        let mut edges_required = 0;
        for e in edges.iter().filter(|e| e[0] == 3) {
            edges_required += usize::from(ufa.union(e[1], e[2]) | ufb.union(e[1], e[2]));
        }
        for e in edges.iter().filter(|e| e[0] == 1) {
            edges_required += usize::from(ufa.union(e[1], e[2]));
        }
        for e in edges.iter().filter(|e| e[0] == 2) {
            edges_required += usize::from(ufb.union(e[1], e[2]));
        }
        if ufa.num_of_connected_components == 1 && ufb.num_of_connected_components == 1 {
            (edges.len() - edges_required) as _
        } else {
            -1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let n = 4;
        let edges = vec![
            vec![3, 1, 2],
            vec![3, 2, 3],
            vec![1, 1, 3],
            vec![1, 2, 4],
            vec![1, 1, 2],
            vec![2, 3, 4],
        ];
        assert_eq!(Solution::max_num_edges_to_remove(n, edges), 2);

        let n = 4;
        let edges = vec![vec![3, 1, 2], vec![3, 2, 3], vec![1, 1, 4], vec![2, 1, 4]];
        assert_eq!(Solution::max_num_edges_to_remove(n, edges), 0);

        let n = 4;
        let edges = vec![vec![3, 2, 3], vec![1, 1, 2], vec![2, 3, 4]];
        assert_eq!(Solution::max_num_edges_to_remove(n, edges), -1);
    }
}
