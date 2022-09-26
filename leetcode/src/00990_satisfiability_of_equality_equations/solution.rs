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
    pub fn equations_possible(equations: Vec<String>) -> bool {
        let ord = |x| x as usize;
        let equations = equations
            .into_iter()
            .map(|e| e.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let mut uf =
            equations
                .iter()
                .filter(|e| e[1] == '=')
                .fold(UnionFind::new(128), |mut uf, e| {
                    uf.union(ord(e[0]), ord(e[3]));
                    uf
                });
        !equations
            .iter()
            .filter(|e| e[1] == '!')
            .any(|e| uf.find(ord(e[0])) == uf.find(ord(e[3])))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let equations = vec![String::from("a==b"), String::from("b!=a")];
        assert!(!Solution::equations_possible(equations));

        let equations = vec![String::from("b==a"), String::from("a==b")];
        assert!(Solution::equations_possible(equations));
    }
}
