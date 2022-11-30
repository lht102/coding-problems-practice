use std::collections::BTreeMap;

struct Solution;

impl Solution {
    pub fn find_winners(matches: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut loss_freq = matches
            .iter()
            .fold(BTreeMap::<i32, usize>::new(), |mut map, m| {
                *map.entry(m[1]).or_default() += 1;
                map
            });
        for m in matches.iter() {
            loss_freq.entry(m[0]).or_default();
        }
        loss_freq
            .into_iter()
            .fold(vec![vec![], vec![]], |mut res, (player, cnt)| {
                if cnt < 2 {
                    res[cnt].push(player);
                }
                res
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let matches = vec![
            vec![1, 3],
            vec![2, 3],
            vec![3, 6],
            vec![5, 6],
            vec![5, 7],
            vec![4, 5],
            vec![4, 8],
            vec![4, 9],
            vec![10, 4],
            vec![10, 9],
        ];
        assert_eq!(
            Solution::find_winners(matches),
            vec![vec![1, 2, 10], vec![4, 5, 7, 8]]
        );

        let matches = vec![vec![2, 3], vec![1, 3], vec![5, 4], vec![6, 4]];
        assert_eq!(
            Solution::find_winners(matches),
            vec![vec![1, 2, 5, 6], vec![]]
        );
    }
}
