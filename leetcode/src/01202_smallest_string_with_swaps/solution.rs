use std::collections::HashMap;
use std::iter::FromIterator;

struct Solution;

struct UnionFind {
    p: Vec<usize>,
    r: Vec<usize>,
}

impl UnionFind {
    pub fn new(n: usize) -> Self {
        Self {
            p: Vec::from_iter(0..n),
            r: vec![0; n],
        }
    }

    pub fn find(&mut self, v: usize) -> usize {
        if v == self.p[v] {
            return v;
        }
        self.p[v] = self.find(self.p[v]);
        self.p[v]
    }

    pub fn union(&mut self, a: usize, b: usize) {
        let mut u = self.find(a);
        let mut v = self.find(b);
        if u == v {
            return;
        }
        if self.r[u] < self.r[v] {
            std::mem::swap(&mut u, &mut v);
        }
        self.p[v] = u;
        if self.r[u] == self.r[v] {
            self.r[u] += 1;
        }
    }
}

impl Solution {
    pub fn smallest_string_with_swaps(s: String, pairs: Vec<Vec<i32>>) -> String {
        let n = s.len();
        let mut uf = UnionFind::new(n);
        for p in &pairs {
            uf.union(p[0] as usize, p[1] as usize);
        }
        let mut root_to_nodes: HashMap<usize, Vec<usize>> = HashMap::new();
        for i in 0..n {
            root_to_nodes.entry(uf.find(i)).or_default().push(i);
        }
        let mut res: Vec<char> = vec![' '; n];
        let s: Vec<char> = s.chars().collect();
        for nodes in root_to_nodes.values() {
            let mut chs = Vec::new();
            for &i in nodes {
                chs.push(s[i]);
            }
            chs.sort_unstable();
            for i in 0..nodes.len() {
                res[nodes[i]] = chs[i];
            }
        }
        res.into_iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let s = String::from("dcab");
        let pairs = vec![vec![0, 3], vec![1, 2], vec![0, 2]];
        assert_eq!(
            Solution::smallest_string_with_swaps(s, pairs),
            String::from("abcd")
        );

        let s = String::from("cba");
        let pairs = vec![vec![0, 1], vec![1, 2]];
        assert_eq!(
            Solution::smallest_string_with_swaps(s, pairs),
            String::from("abc")
        );
    }
}
