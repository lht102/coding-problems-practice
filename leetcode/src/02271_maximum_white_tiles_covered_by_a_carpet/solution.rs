struct Solution;

impl Solution {
    pub fn maximum_white_tiles(tiles: Vec<Vec<i32>>, carpet_len: i32) -> i32 {
        let mut tiles = tiles;
        tiles.sort_unstable_by(|a, b| a[0].cmp(&b[0]));
        let psum = std::iter::once(0)
            .chain(tiles.iter().scan(0, |sum, tile| {
                *sum += tile[1] - tile[0] + 1;
                Some(*sum)
            }))
            .collect::<Vec<_>>();
        let mut res = 0;
        for (i, tile) in tiles.iter().enumerate() {
            let (start, end) = (tile[0], tile[1]);
            let max_end = start + carpet_len - 1;
            if end >= max_end {
                return carpet_len;
            }
            let idx = tiles.partition_point(|tile| tile[0] <= max_end);
            res = res.max(psum[idx] - psum[i] - 0.max(tiles[idx - 1][1] - max_end));
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let tiles = vec![
            vec![1, 5],
            vec![10, 11],
            vec![12, 18],
            vec![20, 25],
            vec![30, 32],
        ];
        let carpet_len = 10;
        assert_eq!(Solution::maximum_white_tiles(tiles, carpet_len), 9);

        let tiles = vec![vec![1, 1], vec![10, 11]];
        let carpet_len = 2;
        assert_eq!(Solution::maximum_white_tiles(tiles, carpet_len), 2);
    }
}
