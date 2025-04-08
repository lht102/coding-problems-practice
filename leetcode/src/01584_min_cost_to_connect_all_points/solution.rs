use std::cmp::Ordering;
use std::iter::FromIterator;

struct Solution;

struct UnionFind {
    p: Vec<i32>,
    sz: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        Self {
            p: Vec::from_iter(0..n as i32),
            sz: vec![1; n],
        }
    }

    fn find(&mut self, v: i32) -> i32 {
        let idx = v as usize;
        if v == self.p[idx] {
            return v;
        }
        self.p[idx] = self.find(self.p[idx]);
        self.p[idx]
    }

    fn union(&mut self, a: i32, b: i32) {
        let u = self.find(a);
        let v = self.find(b);
        if u == v {
            return;
        }
        if self.sz[u as usize] < self.sz[v as usize] {
            self.p[u as usize] = v;
            self.sz[v as usize] += self.sz[u as usize];
        } else {
            self.p[v as usize] = u;
            self.sz[u as usize] += self.sz[v as usize];
        }
    }

    #[allow(clippy::wrong_self_convention)]
    fn is_connected(&mut self, a: i32, b: i32) -> bool {
        self.find(a) == self.find(b)
    }

    fn size(&mut self, a: i32) -> usize {
        let p = self.find(a) as usize;
        self.sz[p]
    }
}

#[derive(PartialEq, Debug, Clone)]
struct Edge {
    from: i32,
    to: i32,
    cost: i32,
}

impl Edge {
    fn new(from: i32, to: i32, cost: i32) -> Self {
        Self { from, to, cost }
    }
}

impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for Edge {}

impl Ord for Edge {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Solution {
    pub fn min_cost_connect_points(points: Vec<Vec<i32>>) -> i32 {
        let mut edges: Vec<Edge> = Vec::new();
        for i in 0..points.len() {
            for j in 0..i {
                edges.push(Edge::new(
                    i as i32,
                    j as i32,
                    Self::distance((points[i][0], points[i][1]), (points[j][0], points[j][1])),
                ))
            }
        }
        edges.sort_unstable();
        let mut res = 0;
        let mut union_find = UnionFind::new(points.len());
        for e in edges {
            if union_find.is_connected(e.from, e.to) {
                continue;
            }
            union_find.union(e.from, e.to);
            res += e.cost;
            if union_find.size(0) == points.len() {
                break;
            }
        }
        res
    }

    fn distance(p1: (i32, i32), p2: (i32, i32)) -> i32 {
        (p1.0 - p2.0).abs() + (p1.1 - p2.1).abs()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let points = vec![vec![0, 0], vec![2, 2], vec![3, 10], vec![5, 2], vec![7, 0]];
        assert_eq!(Solution::min_cost_connect_points(points), 20);

        let points = vec![vec![3, 12], vec![-2, 5], vec![-4, 1]];
        assert_eq!(Solution::min_cost_connect_points(points), 18);
    }
}
