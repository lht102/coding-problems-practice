use std::iter::FromIterator;

struct Solution;

struct UnionFind {
    p: Vec<i32>,
    sz: Vec<usize>,
    num_of_connected_components: usize,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        Self {
            p: Vec::from_iter(0..n as i32),
            sz: vec![1; n],
            num_of_connected_components: n,
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
        self.num_of_connected_components -= 1;
    }
}

impl Solution {
    pub fn remove_stones(stones: Vec<Vec<i32>>) -> i32 {
        let n = stones.len();
        let mut uf = UnionFind::new(n);
        for i in 0..n {
            for j in 0..n {
                if stones[i][0] == stones[j][0] || stones[i][1] == stones[j][1] {
                    uf.union(i as i32, j as i32);
                }
            }
        }
        (n - uf.num_of_connected_components) as _
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let stones = vec![
            vec![0, 0],
            vec![0, 1],
            vec![1, 0],
            vec![1, 2],
            vec![2, 1],
            vec![2, 2],
        ];
        assert_eq!(Solution::remove_stones(stones), 5);

        let stones = vec![vec![0, 0], vec![0, 2], vec![1, 1], vec![2, 0], vec![2, 2]];
        assert_eq!(Solution::remove_stones(stones), 3);

        let stones = vec![vec![0, 0]];
        assert_eq!(Solution::remove_stones(stones), 0);
    }
}
