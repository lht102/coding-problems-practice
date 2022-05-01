struct Solution;

impl Solution {
    pub fn count_unguarded(m: i32, n: i32, guards: Vec<Vec<i32>>, walls: Vec<Vec<i32>>) -> i32 {
        // -1 -> visited
        // 1 -> guard
        // 2 -> wall
        let m = m as usize;
        let n = n as usize;
        let mut g = vec![vec![0; n]; m];
        let mut vi = vec![vec![false; n]; m];
        for guard in &guards {
            g[guard[0] as usize][guard[1] as usize] = 1;
        }
        for wall in &walls {
            g[wall[0] as usize][wall[1] as usize] = 2;
        }
        let visit_fn = |i: usize, j: usize, flag: &mut bool, visited: &mut Vec<Vec<bool>>| {
            if g[i][j] == 2 {
                *flag = false;
            }
            if g[i][j] == 1 {
                *flag = true;
            }
            if *flag {
                visited[i][j] = true;
            }
        };
        for i in 0..m {
            let mut valid = false;
            for j in 0..n {
                visit_fn(i, j, &mut valid, &mut vi);
            }
            valid = false;
            for j in (0..n).rev() {
                visit_fn(i, j, &mut valid, &mut vi);
            }
        }
        for j in 0..n {
            let mut valid = false;
            for i in 0..m {
                visit_fn(i, j, &mut valid, &mut vi);
            }
            valid = false;
            for i in (0..m).rev() {
                visit_fn(i, j, &mut valid, &mut vi);
            }
        }
        let mut res = 0;
        for i in 0..m {
            for j in 0..n {
                if g[i][j] == 0 && !vi[i][j] {
                    res += 1;
                }
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
        let m = 4;
        let n = 6;
        let guards = vec![vec![0, 0], vec![1, 1], vec![2, 3]];
        let walls = vec![vec![0, 1], vec![2, 2], vec![1, 4]];
        assert_eq!(Solution::count_unguarded(m, n, guards, walls), 7);

        let m = 3;
        let n = 3;
        let guards = vec![vec![1, 1]];
        let walls = vec![vec![0, 1], vec![1, 0], vec![2, 1], vec![1, 2]];
        assert_eq!(Solution::count_unguarded(m, n, guards, walls), 4);
    }
}
