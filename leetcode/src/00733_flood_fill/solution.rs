struct Solution;

impl Solution {
    pub fn flood_fill(image: Vec<Vec<i32>>, sr: i32, sc: i32, new_color: i32) -> Vec<Vec<i32>> {
        if image[sr as usize][sc as usize] == new_color {
            return image;
        }
        let mut image = image;
        let old_color = image[sr as usize][sc as usize];
        Solution::dfs(&mut image, sr, sc, old_color, new_color);
        image
    }

    fn dfs(image: &mut Vec<Vec<i32>>, ci: i32, cj: i32, old_color: i32, new_color: i32) {
        image[ci as usize][cj as usize] = new_color;
        let dirs: &'static [i32] = &[0, 1, 0, -1, 0];
        for i in 0..4 {
            let ni = ci + dirs[i];
            let nj = cj + dirs[i + 1];
            if ni < 0
                || ni >= image.len() as i32
                || nj < 0
                || nj >= image[0].len() as i32
                || image[ni as usize][nj as usize] != old_color
            {
                continue;
            }
            Solution::dfs(image, ni, nj, old_color, new_color);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let image = vec![vec![1, 1, 1], vec![1, 1, 0], vec![1, 0, 1]];
        let sr = 1;
        let sc = 1;
        let new_color = 2;
        assert_eq!(
            Solution::flood_fill(image, sr, sc, new_color),
            vec![vec![2, 2, 2], vec![2, 2, 0], vec![2, 0, 1]]
        );

        let image = vec![vec![0, 0, 0], vec![0, 0, 0]];
        let sr = 0;
        let sc = 0;
        let new_color = 2;
        assert_eq!(
            Solution::flood_fill(image, sr, sc, new_color),
            vec![vec![2, 2, 2], vec![2, 2, 2]],
        );
    }
}
